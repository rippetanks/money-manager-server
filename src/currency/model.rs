
use diesel;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::currency;
use crate::database::MoneyManagerDB;

#[table_name = "currency"]
#[derive(Debug,Clone,Serialize,Deserialize,Queryable,Identifiable)]
pub struct Currency {
    pub id: i16,
    pub description: String
}

// only for insert and update
#[table_name = "currency"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct CurrencyForm {
    pub description: String
}

impl Currency {
    pub fn create(currency: CurrencyForm, conn: &MoneyManagerDB) -> QueryResult<Currency> {
        diesel::insert_into(currency::table)
            .values(&currency)
            .get_result::<Currency>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Currency>> {
        currency::table.load::<Currency>(&**conn)
    }
    pub fn read_by_id(id: i16, conn: &MoneyManagerDB) -> QueryResult<Currency> {
        currency::table.find(id).first::<Currency>(&*(*conn))
    }
    pub fn update(id: i16, currency: &CurrencyForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(currency::table.find(id))
            .set(currency)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i16, conn: &MoneyManagerDB) -> bool {
        diesel::delete(currency::table.find(id))
            .execute(&*(*conn)).is_ok()
    }
}