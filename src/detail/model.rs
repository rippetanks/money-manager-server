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

use crate::schema::detail;
use crate::user::model::User;
use crate::database::MoneyManagerDB;

#[table_name = "detail"]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable)]
pub struct Detail {
    pub id: i64,
    pub description: String,
    pub id_user: Option<i64>
}

// only for insert and update
#[table_name = "detail"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct DetailForm<'a> {
    pub description: &'a str,
    pub id_user: Option<i64>
}

impl Detail {
    pub fn create(form: &DetailForm, conn: &MoneyManagerDB) -> QueryResult<Detail> {
        diesel::insert_into(detail::table)
            .values(form)
            .get_result::<Detail>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Detail>> {
        detail::table.load::<Detail>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Detail> {
        detail::table.find(id).first::<Detail>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_user(user: &User, conn: &MoneyManagerDB) -> QueryResult<Vec<Detail>> {
        detail::table.filter(detail::id_user.eq(user.id))
            .load::<Detail>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(detail: &Detail, form: &DetailForm, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(detail)
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(detail: &Detail, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(detail)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}
