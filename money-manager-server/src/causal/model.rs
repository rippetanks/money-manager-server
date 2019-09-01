
use diesel;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::causal;
use crate::database::MoneyManagerDB;

#[table_name = "causal"]
#[derive(Debug,Clone,Serialize,Deserialize,Queryable,Identifiable)]
pub struct Causal {
    pub id: i64,
    pub description: String,
    pub id_user: Option<i64>
}

// only for insert and update
#[table_name = "causal"]
#[derive(Debug,Insertable,AsChangeset)]
pub struct CausalForm {
    pub description: String,
    pub id_user: Option<i64>
}

impl Causal {
    pub fn create(causal: CausalForm, conn: &MoneyManagerDB) -> QueryResult<Causal> {
        diesel::insert_into(causal::table)
            .values(&causal)
            .get_result::<Causal>(&*(*conn))
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Causal>> {
        causal::table.load::<Causal>(&**conn)
    }
    pub fn read_for_user(id_user: i64, conn: &MoneyManagerDB) -> QueryResult<Vec<Causal>> {
        causal::table.filter(causal::id_user.eq(&id_user))
            .load::<Causal>(&*(*conn))
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Causal> {
        causal::table.find(id).first::<Causal>(&*(*conn))
    }
    pub fn update(id: i64, causal: &CausalForm, conn: &MoneyManagerDB) -> bool {
        diesel::update(causal::table.find(id))
            .set(causal)
            .execute(&*(*conn)).is_ok()
    }
    pub fn delete(id: i64, conn: &MoneyManagerDB) -> bool {
        diesel::delete(causal::table.find(id))
            .execute(&*(*conn)).is_ok()
    }
}