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
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

use crate::schema::user;
use crate::database::MoneyManagerDB;

#[table_name = "user"]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub phone: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub note: Option<String>
}

// only for insert and update
#[table_name = "user"]
#[derive(Debug,Insertable,AsChangeset)]
pub struct UserForm {
    pub name: String,
    pub surname: String,
    pub phone: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub note: Option<String>
}

impl User {
    pub fn create(user: &UserForm, conn: &MoneyManagerDB) -> QueryResult<User> {
        diesel::insert_into(user::table)
            .values(user)
            .get_result::<User>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<User>> {
        user::table.load::<User>(&**conn)
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<User> {
        user::table.find(id).first::<User>(&*(*conn))
    }
    pub fn update(id: i64, user: &UserForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(user::table.find(id))
            .set(user)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i64, conn: &MoneyManagerDB) -> bool {
        diesel::delete(user::table.find(id))
            .execute(&*(*conn)).is_ok()
    }
}
