
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
#[derive(Debug,Clone,Serialize,Deserialize,Queryable,Identifiable,Associations)]
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
pub struct AccountForm {
    pub name: String,
    pub status: bool,
    pub note: Option<String>,
    pub current_balance: f64,
    pub initial_balance: f64,
    pub creation_date: DateTime<Utc>,
    pub id_account_type: i32,
    pub id_currency: i16
}

#[table_name="account_user"]
#[belongs_to(Account, foreign_key = "id_account")]
#[belongs_to(User, foreign_key = "id_user")]
#[derive(Debug,Serialize,Deserialize,Queryable,Insertable,Associations)]
pub struct AccountUser {
    pub id_account: i64,
    pub id_user: i64
}

#[table_name="account_type"]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable)]
pub struct AccountType {
    pub id: i32,
    pub type_: String
}

// only for insert and update
#[table_name="account_type"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct AccountTypeForm {
    pub type_: String
}

impl Account {
    pub fn create(account: AccountForm, conn: &MoneyManagerDB) -> QueryResult<Account> {
        diesel::insert_into(account::table)
            .values(&account)
            .get_result::<Account>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Account>> {
        account::table.load::<Account>(&**conn)
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Account> {
        account::table.find(id).first::<Account>(&*(*conn))
    }
    pub fn read_by_user(id: i64, conn: &MoneyManagerDB) -> QueryResult<Vec<Account>> {
        let ids = account_user::table
            .filter(account_user::id_user.eq(id))
            .select(account_user::id_account);
        /*account::table.inner_join(account_user::table)
            .filter(account_user::id_user.eq(id))
            .load::<Account>(&**conn)*/
        account::table
            .filter(account::id.eq(any(ids)))
            .load::<Account>(&**conn)
    }
    pub fn update(id: i64, account: &AccountForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(account::table.find(id))
            .set(account)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i64, conn: &MoneyManagerDB) -> bool {
        let res = conn.transaction::<(), Error, _>(|| {
            // TODO move this on AccountUser implementation
            diesel::delete(account_user::table
                .filter(account_user::id_account.eq(id)))
                .execute(&**conn)
                .map_err(|e| {
                    warn!("{}", e);
                    e
                })?;
            diesel::delete(account::table.find(id))
                .execute(&*(*conn))
                .map_err(|e| {
                    warn!("{}", e);
                    e
                })?;
            Ok(())
        });
        res.is_ok()
        /*let result = diesel::delete(account_user::table
            .filter(account_user::id_account.eq(id)))
            .execute(&**conn);
        if result.is_ok() {
            diesel::delete(account::table.find(id))
                .execute(&*(*conn))
                .map_err(|e| warn!("{}", e)).is_ok()
        } else {
            result.map_err(|e| warn!("{}", e)).is_ok()
        }*/
    }
}

impl AccountUser {
    pub fn create(au: AccountUser, conn: &MoneyManagerDB) -> bool {
        diesel::insert_into(account_user::table)
            .values(&au)
            .execute(&**conn).is_ok()
    }
    pub fn read_by_user(conn: &MoneyManagerDB, user: &User) -> QueryResult<Vec<AccountUser>> {
        /*AccountUser::belonging_to(user)
            .load::<AccountUser>(&**conn)*/
        account_user::table
            .filter(account_user::id_user.eq(user.id))
            .load::<AccountUser>(&**conn)
    }
    pub fn read_by_account(conn: &MoneyManagerDB, account: &Account) -> QueryResult<Vec<AccountUser>> {
        /*AccountUser::belonging_to(account)
            .load::<AccountUser>(&**conn)*/
        account_user::table
            .filter(account_user::id_account.eq(account.id))
            .load::<AccountUser>(&**conn)
    }
    pub fn read_by_au(conn: &MoneyManagerDB, user: &User, id_account: i64) -> QueryResult<AccountUser> {
        account_user::table
            .filter(account_user::id_user.eq(user.id))
            .filter(account_user::id_account.eq(id_account))
            .first::<AccountUser>(&**conn)
    }
    pub fn delete(conn: &MoneyManagerDB, user: &User, account: &Account) -> bool {
        diesel::delete(account_user::table
            .filter(account_user::id_account.eq(account.id))
            .filter(account_user::id_user.eq(user.id)))
            .execute(&*(*conn)).is_ok()
    }
}

impl AccountType {
    pub fn create(at: AccountTypeForm, conn: &MoneyManagerDB) -> QueryResult<AccountType> {
        diesel::insert_into(account_type::table)
            .values(&at)
            .get_result::<AccountType>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<AccountType>> {
        account_type::table.load::<AccountType>(&**conn)
    }
    pub fn read_by_id(id: i32, conn: &MoneyManagerDB) -> QueryResult<AccountType> {
        account_type::table.find(id).first::<AccountType>(&*(*conn))
    }
    pub fn update(id: i32, at: &AccountTypeForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(account_type::table.find(id))
            .set(at)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i32, conn: &MoneyManagerDB) -> bool {
        diesel::delete(account_type::table.find(id))
            .execute(&*(*conn)).is_ok()
    }
}
