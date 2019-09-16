
use diesel;
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::schema::giro;
use crate::currency::model::Currency;
use crate::account::model::Account;
use crate::database::MoneyManagerDB;

#[table_name = "giro"]
//#[belongs_to(Account, foreign_key = "id_source_account")]
//#[belongs_to(Account, foreign_key = "id_destination_account")]
#[belongs_to(Currency, foreign_key = "id_currency")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Associations)]
pub struct Giro {
    pub id: i64,
    pub id_source_account: i64,
    pub id_destination_account: i64,
    pub data: DateTime<Utc>,
    pub note: Option<String>,
    pub amount: f64,
    pub expense: Option<f64>,
    pub id_currency: i16
}

// only for insert and update
#[table_name = "giro"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct GiroForm<'a> {
    pub id_source_account: i64,
    pub id_destination_account: i64,
    pub data: DateTime<Utc>,
    pub note: Option<&'a str>,
    pub amount: f64,
    pub expense: Option<f64>,
    pub id_currency: i16
}

impl Giro {
    pub fn create(form: &GiroForm, conn: &MoneyManagerDB) -> QueryResult<Giro> {
        diesel::insert_into(giro::table)
            .values(form)
            .get_result::<Giro>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Giro>> {
        giro::table.load::<Giro>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Giro> {
        giro::table.find(id).first::<Giro>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_source(account: &Account, conn: &MoneyManagerDB) -> QueryResult<Vec<Giro>> {
        giro::table.filter(giro::id_source_account.eq(account.id))
            .load::<Giro>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_destination(account: &Account, conn: &MoneyManagerDB) -> QueryResult<Vec<Giro>> {
        giro::table.filter(giro::id_destination_account.eq(account.id))
            .load::<Giro>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(giro: &Giro, form: &GiroForm, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(giro)
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(giro: &Giro, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(giro)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}
