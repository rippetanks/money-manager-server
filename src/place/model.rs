
use diesel;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::place;
use crate::user::model::User;
use crate::database::MoneyManagerDB;

#[table_name = "place"]
#[belongs_to(User, foreign_key = "id_user")]
#[derive(Debug,Clone,Serialize,Deserialize,Queryable,Identifiable,Associations)]
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
pub struct PlaceForm {
    pub name: String,
    pub address: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub note: Option<String>,
    pub id_user: Option<i64>
}

impl Place {
    pub fn create(place: PlaceForm, conn: &MoneyManagerDB) -> QueryResult<Place> {
        diesel::insert_into(place::table)
            .values(&place)
            .get_result::<Place>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Place>> {
        place::table.load::<Place>(&**conn)
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Place> {
        place::table.find(id).first::<Place>(&*(*conn))
    }
    pub fn read_by_user(id: i64, conn: &MoneyManagerDB) -> QueryResult<Vec<Place>> {
        place::table
            .filter(place::id_user.eq(id))
            .load::<Place>(&**conn)
    }
    pub fn update(id: i64, place: &PlaceForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(place::table.find(id))
            .set(place)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i64, conn: &MoneyManagerDB) -> bool {
        diesel::delete(place::table.find(id))
            .execute(&*(*conn))
            .map_err(|e| warn!("{}", e)).is_ok()
    }
}
