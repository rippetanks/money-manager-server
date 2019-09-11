
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::giro::model::{Giro, GiroForm};
use crate::account::model::Account;
use crate::user::model::User;

pub mod model;

#[post("/", data = "<giro>", format = "application/json")]
fn create(conn: MoneyManagerDB, giro: Json<GiroForm>, user: User) -> Result<Json<Giro>, Status> {
    debug!("CREATE_GIRO_REQUEST");
    Giro::create(giro.into_inner(), &conn)
        .map(|g| {
            info!("giro create successfully {}", g.id);
            Json(g)
        })
        .map_err(|e| {
            error!("Can not create giro caused by {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Giro>, Status> {
    debug!("READ_ONE_GIRO_REQUEST");
    // a user can access his own giro
    // ...
    Giro::read_by_id(id, &conn)
        .map(Json)
        .map_err(|e| {
            warn!("The user attempts to access giro that maybe does not exist! {}", e);
            Status::NotFound
        })
}

#[get("/account/source/<id>")]
pub fn read_by_source(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<Giro>>, Custom<String>> {
    debug!("READ_BY_ACCOUNT_SOURCE_GIRO_REQUEST");
    let account = Account::read_by_id(id, &conn);
    if account.is_err() {
        return Err(Custom(Status::InternalServerError, String::new()));
    }
    let result = Giro::read_by_source(&account.ok().unwrap(), &conn);
    Giro::unpack(result)
}

#[get("/account/destination/<id>")]
pub fn read_by_destination(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<Giro>>, Custom<String>> {
    debug!("READ_BY_ACCOUNT_DESTINATION_GIRO_REQUEST");
    let account = Account::read_by_id(id, &conn);
    if account.is_err() {
        return Err(Custom(Status::InternalServerError, String::new()));
    }
    let result = Giro::read_by_destination(&account.ok().unwrap(), &conn);
    Giro::unpack(result)
}

#[put("/<id>", data = "<giro>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, giro: Json<GiroForm>, user: User) -> Status {
    debug!("UPDATE_GIRO_REQUEST");
    // check if account can be updated
    // ...
    if Giro::update(id, &giro.into_inner(), &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to update giro but an error occurred!");
        Status::InternalServerError
    }
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Status {
    debug!("DELETE_GIRO_REQUEST");
    // check if causal can be deleted
    // ...
    if Giro::delete(id, &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to delete giro but an error occurred!");
        Status::InternalServerError
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/giro", routes![read_one, read_by_source, read_by_destination, create, update, delete])
}
