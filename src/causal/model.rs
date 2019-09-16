
use diesel;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::causal;
use crate::user::model::User;
use crate::database::MoneyManagerDB;

#[table_name = "causal"]
#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable)]
pub struct Causal {
    pub id: i64,
    pub description: String,
    pub id_user: Option<i64>
}

// only for insert and update
#[table_name = "causal"]
#[derive(Debug,Insertable,AsChangeset)]
pub struct CausalForm<'a> {
    pub description: &'a str,
    pub id_user: Option<i64>
}

impl Causal {
    pub fn create(form: &CausalForm, conn: &MoneyManagerDB) -> QueryResult<Causal> {
        diesel::insert_into(causal::table)
            .values(form)
            .get_result::<Causal>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read(conn: &MoneyManagerDB) -> QueryResult<Vec<Causal>> {
        causal::table.load::<Causal>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_for_user(user: &User, conn: &MoneyManagerDB) -> QueryResult<Vec<Causal>> {
        causal::table.filter(causal::id_user.eq(user.id))
            .or_filter(causal::id_user.is_null())
            .load::<Causal>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn read_by_id(id: i64, conn: &MoneyManagerDB) -> QueryResult<Causal> {
        causal::table.find(id).first::<Causal>(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn update(causal: &Causal, form: &CausalForm, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::update(causal)
            .set(form)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
    pub fn delete(causal: &Causal, conn: &MoneyManagerDB) -> QueryResult<usize> {
        diesel::delete(causal)
            .execute(&*(*conn))
            .map_err(|e| { warn!("{}", e); e })
    }
}