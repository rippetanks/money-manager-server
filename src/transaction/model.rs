
use diesel;
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::schema::{transaction, transaction_type, transaction_detail};
use crate::user::model::User;
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
#[derive(Debug,Clone,Serialize,Deserialize,Queryable,Identifiable,Associations)]
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
pub struct TransactionForm {
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
pub struct TransactionTypeForm {
    #[serde(rename="type")]
    pub type_: String
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
    pub fn create(transaction: TransactionForm, conn: &MoneyManagerDB) -> QueryResult<Transaction> {
        diesel::insert_into(transaction::table)
            .values(&transaction)
            .get_result::<Transaction>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Transaction>> {
        transaction::table.load::<Transaction>(&**conn)
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Transaction> {
        transaction::table.find(id).first::<Transaction>(&*(*conn))
    }
    pub fn read_by_account(id: i64, conn: &MoneyManagerDB) -> QueryResult<Vec<Transaction>> {
        transaction::table
            .filter(transaction::id_account.eq(id))
            .load::<Transaction>(&**conn)
    }
    pub fn update(id: i64, transaction: &TransactionForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(transaction::table.find(id))
            .set(transaction)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i64, conn: &MoneyManagerDB) -> bool {
        /*let result = diesel::delete(account_user::table
            .filter(account_user::id_account.eq(id)))
            .execute(&**conn);
        if result.is_ok() {*/
            diesel::delete(transaction::table.find(id))
                .execute(&*(*conn))
                .map_err(|e| warn!("{}", e)).is_ok()
        /*} else {
            result.map_err(|e| warn!("{}", e)).is_ok()
        }*/
    }
}

impl TransactionDetail {
    pub fn create(td: TransactionDetail, conn: &MoneyManagerDB) -> bool {
        diesel::insert_into(transaction_detail::table)
            .values(&td)
            .execute(&**conn).is_ok()
    }
    pub fn read_by_transaction(conn: &MoneyManagerDB, transaction: &Transaction) -> QueryResult<Vec<TransactionDetail>> {
        TransactionDetail::belonging_to(transaction)
            .load::<TransactionDetail>(&**conn)
    }
    pub fn read_by_detail(conn: &MoneyManagerDB, detail: &Detail) -> QueryResult<Vec<TransactionDetail>> {
        TransactionDetail::belonging_to(detail)
            .load::<TransactionDetail>(&**conn)
    }
    pub fn read_by_td(conn: &MoneyManagerDB, id_detail: i64, id_transaction: i64) -> QueryResult<TransactionDetail> {
        transaction_detail::table
            .filter(transaction_detail::id_detail.eq(id_detail))
            .filter(transaction_detail::id_transaction.eq(id_transaction))
            .first::<TransactionDetail>(&**conn)
    }
    pub fn update(td: &TransactionDetail, conn: &MoneyManagerDB) -> bool {
        diesel::update(transaction_detail::table
            .filter(transaction_detail::id_detail.eq(td.id_detail))
            .filter(transaction_detail::id_transaction.eq(td.id_transaction)))
            .set(td)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete_by_td(conn: &MoneyManagerDB, td: &TransactionDetail) -> bool {
        diesel::delete(td).execute(&**conn).is_ok()
    }
    pub fn delete(conn: &MoneyManagerDB, transaction: &Transaction, detail: &Detail) -> bool {
        diesel::delete(transaction_detail::table
            .filter(transaction_detail::id_transaction.eq(transaction.id))
            .filter(transaction_detail::id_detail.eq(detail.id)))
            .execute(&*(*conn)).is_ok()
    }
}

impl TransactionType {
    pub fn create(tt: TransactionTypeForm, conn: &MoneyManagerDB) -> QueryResult<TransactionType> {
        diesel::insert_into(transaction_type::table)
            .values(&tt)
            .get_result::<TransactionType>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<TransactionType>> {
        transaction_type::table.load::<TransactionType>(&**conn)
    }
    pub fn read_by_id(id: i32, conn: &MoneyManagerDB) -> QueryResult<TransactionType> {
        transaction_type::table.find(id).first::<TransactionType>(&*(*conn))
    }
    pub fn update(id: i32, tt: &TransactionTypeForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(transaction_type::table.find(id))
            .set(tt)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i32, conn: &MoneyManagerDB) -> bool {
        diesel::delete(transaction_type::table.find(id))
            .execute(&*(*conn)).is_ok()
    }
}
