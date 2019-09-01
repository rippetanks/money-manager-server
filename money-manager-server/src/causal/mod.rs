
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::auth::auth::ApiKey;
use crate::causal::model::{Causal, CausalForm};
use crate::user::model::User;
use std::borrow::Borrow;

pub mod model;

#[derive(Debug,Deserialize)]
struct CausalJSON<'a> {
    pub description: &'a str
}

#[post("/", data = "<causal>", format = "application/json")]
fn create(conn: MoneyManagerDB, causal: Json<CausalJSON>, user: User) -> Result<Json<Causal>, Status> {
    debug!("CREATE_CAUSAL_REQUEST");
    let insert = CausalForm {
        description: causal.description.to_string(),
        id_user: Some(user.id),
    };
    Causal::create(insert, &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

/* DISABLED FOR SECURITY REASON
#[get("/")]
fn read(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Causal>>, Custom<String>> {
    debug!("READ_CAUSAL_REQUEST");
    let result = Causal::read(&conn);
    Causal::unpack(result)
}
*/

#[get("/user")]
fn read_for_user(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Causal>>, Custom<String>> {
    debug!("READ_FOR_USER_CAUSAL_REQUEST");
    let result = Causal::read_for_user(user.id, &conn);
    Causal::unpack(result)
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Causal>, Status> {
    debug!("READ_ONE_CAUSAL_REQUEST");
    let result = Causal::read_by_id(id, &conn);
    match &result {
        // a user can access his own causals or the default ones
        Ok(result) if result.id_user.is_none() || (result.id_user.unwrap() == user.id) => {
            Ok(Json((*result).clone()))
        },
        Ok(result) => {
            warn!("The user attempts to access causal that does not belong to it! {:?}", result);
            Err(Status::Forbidden)
        }
        Err(_) => {
            warn!("The user attempts to access causal that does not exist!");
            Err(Status::NotFound)
        }
    }
}

#[put("/<id>", data = "<causal>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, causal: Json<CausalJSON>, user: User) -> Status {
    debug!("UPDATE_CAUSAL_REQUEST");
    let mut update = CausalForm {
        description: causal.description.to_string(),
        id_user: Some(user.id)
    };
    // check if causal can be updated
    let causal = Causal::read_by_id(id, &conn)
        .map(|c| c.id_user.is_some() && c.id_user.unwrap() == user.id);
    match causal {
        Ok(causal) if causal => {
            if Causal::update(id, &update, &conn) {
                Status::NoContent
            } else {
                warn!("The user attempts to update causal but an error occurred!");
                Status::InternalServerError
            }
        },
        Ok(_) => {
            warn!("The user attempts to update causal that does not belong to it! {}", id);
            Status::Forbidden
        },
        Err(e) => {
            warn!("The user attempts to update causal that does not exists! {:?}", e);
            Status::NotFound
        }
    }
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Status {
    debug!("DELETE_CAUSAL_REQUEST");
    // check if causal can be deleted
    let causal = Causal::read_by_id(id, &conn)
        .map(|c| c.id_user.is_some() && c.id_user.unwrap() == user.id);
    match causal {
        Ok(causal) if causal => {
            if Causal::delete(id, &conn) {
                Status::NoContent
            } else {
                warn!("The user attempts to delete causal but an error occurred!");
                Status::InternalServerError
            }
        },
        Ok(_) => {
            warn!("The user attempts to delete causal that does not belong to it! {}", id);
            Status::Forbidden
        }
        Err(e) => {
            warn!("The user attempts to delete causal that does not exists! {:?}", e);
            Status::NotFound
        }
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/causal", routes![read_one, read_for_user, create, update, delete])
}
