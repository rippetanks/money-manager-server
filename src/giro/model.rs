
use diesel;
use diesel::prelude::*;
use diesel::pg::expression::dsl::any;
use chrono::{DateTime, Utc};
use diesel::result::Error;
use serde::{Serialize, Deserialize};

use crate::schema::giro;
use crate::user::model::User;
use crate::currency::model::Currency;
use crate::account::model::Account;
use crate::database::MoneyManagerDB;

#[table_name = "giro"]
//#[belongs_to(Account, foreign_key = "id_source_account")]
//#[belongs_to(Account, foreign_key = "id_destination_account")]
#[belongs_to(Currency, foreign_key = "id_currency")]
#[derive(Debug,Clone,Serialize,Deserialize,Queryable,Identifiable,Associations)]
pub struct Giro {
    pub id: i64,
    pub id_source_account: i64,
    pub id_destination_account: i64,
    pub data: chrono::DateTime<Utc>,
    pub note: Option<String>,
    pub amount: f64,
    pub expense: Option<f64>,
    pub id_currency: i16
}

// only for insert and update
#[table_name = "giro"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct GiroForm {
    pub id_source_account: i64,
    pub id_destination_account: i64,
    pub data: chrono::DateTime<Utc>,
    pub note: Option<String>,
    pub amount: f64,
    pub expense: Option<f64>,
    pub id_currency: i16
}

impl Giro {
    pub fn create(giro: GiroForm, conn: &MoneyManagerDB) -> QueryResult<Giro> {
        diesel::insert_into(giro::table)
            .values(&giro)
            .get_result::<Giro>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Giro>> {
        giro::table.load::<Giro>(&**conn)
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Giro> {
        giro::table.find(id).first::<Giro>(&*(*conn))
    }
    pub fn read_by_source(account: &Account, conn: &MoneyManagerDB) -> QueryResult<Vec<Giro>> {
        giro::table.filter(giro::id_source_account.eq(account.id))
            .load::<Giro>(&**conn)
        /*Giro::belonging_to(giro::id_source_account, account)
            .load::<Giro>(&**conn)*/
    }
    pub fn read_by_destination(account: &Account, conn: &MoneyManagerDB) -> QueryResult<Vec<Giro>> {
        giro::table.filter(giro::id_destination_account.eq(account.id))
            .load::<Giro>(&**conn)
    }
    pub fn update(id: i64, giro: &GiroForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(giro::table.find(id))
            .set(giro)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i64, conn: &MoneyManagerDB) -> bool {
        diesel::delete(giro::table.find(id))
            .execute(&**conn).is_ok()
    }
}
