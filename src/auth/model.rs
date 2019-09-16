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

use crate::schema::auth;
use crate::user::model::User;
use crate::database::MoneyManagerDB;

#[table_name = "auth"]
#[belongs_to(User, foreign_key = "id")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Associations,Insertable,AsChangeset)]
pub struct Auth {
    pub id: i64,
    pub email: String,
    pub iteration: i16,
    pub salt: String,
    pub stored_key: String,
    pub last_login: Option<DateTime<Utc>>
}

impl Auth {
    pub fn create(form: &Auth, conn: &MoneyManagerDB) -> QueryResult<Auth> {
        diesel::insert_into(auth::table)
            .values(form)
            .get_result::<Auth>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Auth>> {
        auth::table.load::<Auth>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Auth> {
        auth::table.find(id).first::<Auth>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_user(user: &User, conn: &MoneyManagerDB) -> QueryResult<Auth> {
        Auth::belonging_to(user).first::<Auth>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_email(email: &str, conn: &MoneyManagerDB) -> QueryResult<Auth> {
        auth::table
            .filter(auth::email.eq(email))
            .first(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(id: i64, form: &Auth, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(auth::table.find(id))
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update_last_login(id: i64, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(auth::table.find(id))
            .set(auth::last_login.eq(Utc::now()))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(id: i64, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(auth::table.find(id))
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    ///
    /// Not all info can be returned.
    pub fn mask(auth: &mut Auth) {
        auth.stored_key = String::new();
        auth.salt = String::new();
        auth.iteration = 0;
    }
}
