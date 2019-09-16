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
use serde::{Serialize, Deserialize};

use crate::schema::currency;
use crate::database::MoneyManagerDB;

#[table_name = "currency"]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable)]
pub struct Currency {
    pub id: i16,
    pub description: String
}

// only for insert and update
#[table_name = "currency"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct CurrencyForm<'a> {
    pub description: &'a str
}

impl Currency {
    pub fn create(form: &CurrencyForm, conn: &MoneyManagerDB) -> QueryResult<Currency> {
        diesel::insert_into(currency::table)
            .values(form)
            .get_result::<Currency>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Currency>> {
        currency::table.load::<Currency>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i16, conn: &MoneyManagerDB) -> QueryResult<Currency> {
        currency::table.find(id).first::<Currency>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(currency: &Currency, form: &CurrencyForm, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(currency)
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(currency: &Currency, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(currency)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}
