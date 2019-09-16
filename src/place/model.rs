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

use crate::schema::place;
use crate::user::model::User;
use crate::database::MoneyManagerDB;

#[table_name = "place"]
#[belongs_to(User, foreign_key = "id_user")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Associations)]
pub struct Place {
    pub id: i64,
    pub name: String,
    pub address: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub note: Option<String>,
    pub id_user: Option<i64>
}

// only for insert and update
#[table_name = "place"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct PlaceForm<'a> {
    pub name: &'a str,
    pub address: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub note: Option<String>,
    pub id_user: Option<i64>
}

impl Place {
    pub fn create(form: &PlaceForm, conn: &MoneyManagerDB) -> QueryResult<Place> {
        diesel::insert_into(place::table)
            .values(form)
            .get_result::<Place>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Place>> {
        place::table.load::<Place>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Place> {
        place::table.find(id).first::<Place>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_user(user: &User, conn: &MoneyManagerDB) -> QueryResult<Vec<Place>> {
        place::table
            .filter(place::id_user.eq(user.id))
            .load::<Place>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(place: &Place, form: &PlaceForm, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(place)
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(place: &Place, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(place)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}
