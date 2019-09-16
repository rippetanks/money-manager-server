/*
    Copyright (C) 2019  Simone Martelli

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use diesel;
use diesel::prelude::*;
use diesel::pg::expression::dsl::any;
use chrono::{DateTime, Utc};
use diesel::result::Error;
use serde::{Serialize, Deserialize};

use crate::schema::{account, account_user, account_type};
use crate::user::model::User;
use crate::database::MoneyManagerDB;

#[table_name = "account"]
#[belongs_to(AccountType, foreign_key = "id_account_type")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Associations)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub status: bool,
    pub note: Option<String>,
    pub current_balance: f64,
    pub initial_balance: f64,
    pub creation_date: DateTime<Utc>,
    pub id_account_type: i32,
    pub id_currency: i16
}

// only for insert and update
#[table_name = "account"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct AccountForm<'a> {
    pub name: &'a str,
    pub status: bool,
    pub note: Option<&'a str>,
    pub current_balance: f64,
    pub initial_balance: f64,
    pub creation_date: DateTime<Utc>,
    pub id_account_type: i32,
    pub id_currency: i16
}

#[table_name="account_user"]
#[primary_key(id_account,id_user)]
#[belongs_to(Account, foreign_key = "id_account")]
#[belongs_to(User, foreign_key = "id_user")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Insertable,Associations)]
pub struct AccountUser {
    pub id_account: i64,
    pub id_user: i64
}

#[table_name="account_type"]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable)]
pub struct AccountType {
    pub id: i32,
    #[serde(rename="type")]
    pub type_: String
}

// only for insert and update
#[table_name="account_type"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct AccountTypeForm<'a> {
    #[serde(rename="type")]
    pub type_: &'a str
}

impl Account {
    pub fn create(form: &AccountForm, conn: &MoneyManagerDB) -> QueryResult<Account> {
        diesel::insert_into(account::table)
            .values(form)
            .get_result::<Account>(&*(*conn))
            .map_err(|e| { error!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Account>> {
        account::table.load::<Account>(&**conn)
            .map_err(|e| { error!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Account> {
        account::table.find(id).first::<Account>(&*(*conn))
            .map_err(|e| { error!("{}", e); e })
    }
    pub fn read_by_user(user: &User, conn: &MoneyManagerDB) -> QueryResult<Vec<Account>> {
        let ids = AccountUser::belonging_to(user)
            .select(account_user::id_account)
            .load::<i64>(&*(*conn))
            .map_err(|e| { error!("{}", e); e })?;
        account::table
            .filter(account::id.eq(any(ids)))
            .load::<Account>(&*(*conn))
            .map_err(|e| { error!("{}", e); e })
    }
    pub fn update(account: &Account, form: &AccountForm, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(account)
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { error!("{}", e); e })
    }
    pub fn delete(account: &Account, conn: &MoneyManagerDB) -> QueryResult<usize> {
        conn.transaction::<usize, Error, _>(|| {
            AccountUser::delete_by_account(account, conn)?; // TODO mettere fuori
            // TODO delete all the other related data
            diesel::delete(account)
                .execute(&*(*conn))
                .map_err(|e| { warn!("{}", e); e })
        })
    }
}

impl AccountUser {
    pub fn create(form: &AccountUser, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::insert_into(account_user::table)
            .values(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_user(conn: &MoneyManagerDB, user: &User) -> QueryResult<Vec<AccountUser>> {
        AccountUser::belonging_to(user)
            .load::<AccountUser>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_account(conn: &MoneyManagerDB, account: &Account) -> QueryResult<Vec<AccountUser>> {
        AccountUser::belonging_to(account)
            .load::<AccountUser>(&**conn)
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_au(conn: &MoneyManagerDB, user: &User, account: &Account) -> QueryResult<AccountUser> {
        AccountUser::read_for_check(conn, user, account.id)
    }
    pub fn read_for_check(conn: &MoneyManagerDB, user: &User, id_account: i64) -> QueryResult<AccountUser> {
        account_user::table
            .filter(account_user::id_user.eq(user.id))
            .filter(account_user::id_account.eq(id_account))
            .first::<AccountUser>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(conn: &MoneyManagerDB, user: &User, account: &Account) -> QueryResult<usize> {
        diesel::delete(account_user::table
            .filter(account_user::id_account.eq(account.id))
            .filter(account_user::id_user.eq(user.id)))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete_by_account(account: &Account, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(account_user::table
            .filter(account_user::id_account.eq(account.id)))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete_by_user(user: &User, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(account_user::table
            .filter(account_user::id_user.eq(user.id)))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}

impl AccountType {
    pub fn create(form: &AccountTypeForm, conn: &MoneyManagerDB) -> QueryResult<AccountType> {
        diesel::insert_into(account_type::table)
            .values(form)
            .get_result::<AccountType>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<AccountType>> {
        account_type::table.load::<AccountType>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i32, conn: &MoneyManagerDB) -> QueryResult<AccountType> {
        account_type::table.find(id).first::<AccountType>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(at: &AccountType, form: &AccountTypeForm, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(at)
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(at: &AccountType, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(at)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}
