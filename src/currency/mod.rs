
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::auth::auth::ApiKey;
use crate::user::model::User;
use crate::currency::model::{Currency, CurrencyForm};

pub mod model;

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[post("/", data = "<currency>", format = "application/json")]
fn create(conn: MoneyManagerDB, currency: Json<CurrencyForm>, user: User) -> Result<Json<Currency>, Status> {
    debug!("CREATE_CURRENCY_REQUEST");
    Currency::create(currency.into_inner(), &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/")]
fn read(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Currency>>, Custom<String>> {
    debug!("READ_CURRENCY_REQUEST");
    let result = Currency::read(&conn);
    Currency::unpack(result)
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i16, user: User) -> Result<Json<Currency>, Status> {
    debug!("READ_ONE_CURRENCY_REQUEST");
    let result = Currency::read_by_id(id, &conn);
    result
        .map(Json)
        .map_err(|e| {
            warn!("The user attempts to access currency that does not exist!");
            Status::NotFound
        })
}

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[put("/<id>", data = "<currency>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i16, currency: Json<CurrencyForm>, user: User) -> Status {
    debug!("UPDATE_CURRENCY_REQUEST");
    if Currency::update(id, &currency, &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to update currency but an error occurred!");
        Status::InternalServerError
    }
}

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i16, user: User) -> Status {
    debug!("DELETE_CURRENCY_REQUEST");
    if Currency::delete(id, &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to delete currency but an error occurred!");
        Status::InternalServerError
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/currency", routes![read, read_one])
}
