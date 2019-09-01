
use diesel;
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::schema::auth;
use crate::user::model::User;
use crate::database::MoneyManagerDB;
use std::time::SystemTime;

#[table_name = "auth"]
#[belongs_to(User, foreign_key = "id")]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Insertable,AsChangeset,Associations)]
pub struct Auth {
    pub id: i64,
    pub email: String,
    pub iteration: i16,
    pub salt: String,
    pub stored_key: String,
    pub last_login: Option<DateTime<Utc>>
}

impl Auth {
    pub fn create(auth: Auth, conn: &MoneyManagerDB) -> QueryResult<Auth> {
        diesel::insert_into(auth::table)
            .values(&auth)
            .get_result::<Auth>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Auth>> {
        auth::table.load::<Auth>(&**conn)
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Auth> {
        auth::table.find(id).first::<Auth>(&*(*conn))
    }
    pub fn read_by_user(user: &User, conn: &MoneyManagerDB) -> QueryResult<Auth> {
        Auth::belonging_to(user).first::<Auth>(&*(*conn))
    }
    pub fn read_by_email(email: &String, conn: &MoneyManagerDB) -> QueryResult<Auth> {
        auth::table
            .filter(auth::email.eq(email))
            .first(&*(*conn))
    }
    pub fn update(id: i64, auth: &Auth, conn: &MoneyManagerDB) -> bool {
        diesel::update(auth::table.find(id))
            .set(auth)
            .execute(&*(*conn)).is_ok()
    }
    pub fn update_last_login(auth: &Auth, conn: &MoneyManagerDB) -> bool {
        diesel::update(auth::table.filter(auth::id.eq(auth.id)))
            .set(auth::last_login.eq(Utc::now()))
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i64, conn: &MoneyManagerDB) -> bool {
        diesel::delete(auth::table.find(id))
            .execute(&*(*conn)).is_ok()
    }
    ///
    ///
    pub fn mask(auth: &mut Auth) {
        auth.stored_key = String::new();
        auth.salt = String::new();
        auth.iteration = 0;
    }
}
