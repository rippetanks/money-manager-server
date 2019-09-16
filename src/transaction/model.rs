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
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::schema::{transaction, transaction_type, transaction_detail};
use crate::account::model::Account;
use crate::currency::model::Currency;
use crate::causal:: model::Causal;
use crate::detail::model::Detail;
use crate::database::MoneyManagerDB;

#[table_name = "transaction"]
#[belongs_to(Account, foreign_key = "id_account")]
#[belongs_to(TransactionType, foreign_key="id_transaction_type")]
#[belongs_to(Currency, foreign_key="id_currency")]
#[belongs_to(Causal, foreign_key="id_causal")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Associations)]
pub struct Transaction {
    pub id: i64,
    pub id_account: i64,
    pub id_transaction_type: i32,
    pub id_place: Option<i64>,
    pub id_beneficiary: Option<i64>,
    pub note: Option<String>,
    pub amount: f64,
    pub data: DateTime<Utc>,
    pub id_currency: i16,
    pub expense: Option<f64>,
    pub id_causal: i64
}

// only for insert and update
#[table_name = "transaction"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct TransactionForm<'a> {
    pub id_account: i64,
    pub id_transaction_type: i32,
    pub id_place: Option<i64>,
    pub id_beneficiary: Option<i64>,
    pub note: Option<&'a str>,
    pub amount: f64,
    pub data: DateTime<Utc>,
    pub id_currency: i16,
    pub expense: Option<f64>,
    pub id_causal: i64
}

#[table_name="transaction_type"]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable)]
pub struct TransactionType {
    pub id: i32,
    #[serde(rename="type")]
    pub type_: String
}

// only for insert and update
#[table_name="transaction_type"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct TransactionTypeForm<'a> {
    #[serde(rename="type")]
    pub type_: &'a str
}

#[table_name="transaction_detail"]
#[primary_key(id_detail,id_transaction)]
#[belongs_to(Detail, foreign_key = "id_detail")]
#[belongs_to(Transaction, foreign_key="id_transaction")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Insertable,AsChangeset,Associations)]
pub struct TransactionDetail {
    pub id_detail: i64,
    pub id_transaction: i64,
    pub amount: Option<i16>
}

impl Transaction {
    pub fn create(form: &TransactionForm, conn: &MoneyManagerDB) -> QueryResult<Transaction> {
        diesel::insert_into(transaction::table)
            .values(form)
            .get_result::<Transaction>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Transaction>> {
        transaction::table.load::<Transaction>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Transaction> {
        transaction::table.find(id).first::<Transaction>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_account(account: &Account, conn: &MoneyManagerDB) -> QueryResult<Vec<Transaction>> {
        transaction::table
            .filter(transaction::id_account.eq(account.id))
            .load::<Transaction>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(transaction: &Transaction, form: &TransactionForm, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(transaction)
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(transaction: &Transaction, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(transaction)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}

impl TransactionDetail {
    pub fn create(td: &TransactionDetail, conn: &MoneyManagerDB) -> bool {
        diesel::insert_into(transaction_detail::table)
            .values(td)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e }).is_ok()
    }
    pub fn read_by_transaction(conn: &MoneyManagerDB, transaction: &Transaction) -> QueryResult<Vec<TransactionDetail>> {
        TransactionDetail::belonging_to(transaction)
            .load::<TransactionDetail>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_detail(conn: &MoneyManagerDB, detail: &Detail) -> QueryResult<Vec<TransactionDetail>> {
        TransactionDetail::belonging_to(detail)
            .load::<TransactionDetail>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_td(conn: &MoneyManagerDB, detail: &Detail, transaction: &Transaction) -> QueryResult<TransactionDetail> {
        transaction_detail::table
            .filter(transaction_detail::id_detail.eq(detail.id))
            .filter(transaction_detail::id_transaction.eq(transaction.id))
            .first::<TransactionDetail>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(td: &TransactionDetail, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(td)
            .set(td)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(conn: &MoneyManagerDB, td: &TransactionDetail) -> QueryResult<usize> {
        diesel::delete(td)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete_by_td(conn: &MoneyManagerDB, transaction: &Transaction, detail: &Detail) -> QueryResult<usize> {
        diesel::delete(transaction_detail::table
            .filter(transaction_detail::id_transaction.eq(transaction.id))
            .filter(transaction_detail::id_detail.eq(detail.id)))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}

impl TransactionType {
    pub fn create(form: &TransactionTypeForm, conn: &MoneyManagerDB) -> QueryResult<TransactionType> {
        diesel::insert_into(transaction_type::table)
            .values(form)
            .get_result::<TransactionType>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<TransactionType>> {
        transaction_type::table.load::<TransactionType>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i32, conn: &MoneyManagerDB) -> QueryResult<TransactionType> {
        transaction_type::table.find(id).first::<TransactionType>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(tt: &TransactionType, form: &TransactionTypeForm, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(tt)
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(tt: &TransactionType, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(tt)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}
