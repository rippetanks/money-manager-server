

use diesel;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::detail;
use crate::user::model::User;
use crate::database::MoneyManagerDB;

#[table_name = "detail"]
#[derive(Debug,Clone,Serialize,Deserialize,Queryable,Identifiable)]
pub struct Detail {
    pub id: i64,
    pub description: String,
    pub id_user: Option<i64>
}

// only for insert and update
#[table_name = "detail"]
#[derive(Debug,Deserialize,Insertable,AsChangeset)]
pub struct DetailForm {
    pub description: String,
    pub id_user: Option<i64>
}

impl Detail {
    pub fn create(detail: DetailForm, conn: &MoneyManagerDB) -> QueryResult<Detail> {
        diesel::insert_into(detail::table)
            .values(&detail)
            .get_result::<Detail>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Detail>> {
        detail::table.load::<Detail>(&**conn)
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Detail> {
        detail::table.find(id).first::<Detail>(&*(*conn))
    }
    pub fn read_by_user(id: i64, conn: &MoneyManagerDB) -> QueryResult<Vec<Detail>> {
        detail::table
            .filter(detail::id_user.eq(id))
            .load::<Detail>(&**conn)
    }
    pub fn update(id: i64, detail: &DetailForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(detail::table.find(id))
            .set(detail)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i64, conn: &MoneyManagerDB) -> bool {
        diesel::delete(detail::table.find(id))
            .execute(&*(*conn))
            .map_err(|e| warn!("{}", e)).is_ok()
    }
}

