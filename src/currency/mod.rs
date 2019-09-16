
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::user::model::User;
use crate::currency::model::{Currency, CurrencyForm};

pub mod model;

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<CurrencyForm>, _user: User) -> Result<Json<Currency>, Status> {
    debug!("CREATE_CURRENCY_REQUEST");
    Currency::create(&json.into_inner(), &conn)
        .map(|result| {
            info!("currency create successfully: {}", result.id);
            Json(result)
        })
        .map_err(|e| {
            error!("Can not create currency: {}", e);
            Status::InternalServerError
        })
}

#[get("/")]
fn read(conn: MoneyManagerDB, _user: User) -> Result<Json<Vec<Currency>>, Custom<String>> {
    debug!("READ_CURRENCY_REQUEST");
    let result = Currency::read(&conn);
    Currency::unpack(result)
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i16, _user: User) -> Result<Json<Currency>, Status> {
    debug!("READ_ONE_CURRENCY_REQUEST");
    let result = get_by_id(id, &conn)?;
    Ok(Json(result))
}

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[put("/<id>", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i16, json: Json<CurrencyForm>, _user: User) -> Result<Status, Status> {
    debug!("UPDATE_CURRENCY_REQUEST");
    let currency = get_by_id(id, &conn)?;
    let update = Currency::update(&currency, &json.into_inner(), &conn);
    Currency::finalize_update_delete(update)
}

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i16, _user: User) -> Result<Status, Status> {
    debug!("DELETE_CURRENCY_REQUEST");
    let currency = get_by_id(id, &conn)?;
    let delete = Currency::delete(&currency, &conn);
    Currency::finalize_update_delete(delete)
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/currency", routes![read, read_one])
}

// #################################################################################################

fn get_by_id(id: i16, conn: &MoneyManagerDB) -> Result<Currency, Status> {
    Currency::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read currency: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}
