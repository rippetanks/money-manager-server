
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::transaction::model::{TransactionType, TransactionTypeForm};
use crate::user::model::User;

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[post("/", data = "<tt>", format = "application/json")]
fn create(conn: MoneyManagerDB, tt: Json<TransactionTypeForm>, user: User) -> Result<Json<TransactionType>, Status> {
    debug!("CREATE_TRANSACTION_TYPE_REQUEST");
    TransactionType::create(tt.into_inner(), &conn)
        .map(|a| {
            info!("transaction_type create successfully {}", a.id);
            Json(a)
        })
        .map_err(|e| {
            error!("Can not create transaction_type caused by {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i32, user: User) -> Result<Json<TransactionType>, Status> {
    debug!("READ_ONE_TRANSACTION_TYPE_REQUEST");
    TransactionType::read_by_id(id, &conn)
        .map(Json)
        .map_err(|e| {
            error!("Can not read transaction_type by {}", e);
            Status::NotFound
        })
}

#[get("/")]
fn read(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<TransactionType>>, Custom<String>> {
    debug!("READ_ALL_TRANSACTION_TYPE_REQUEST");
    let result = TransactionType::read(&conn);
    TransactionType::unpack(result)
}

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[put("/<id>", data = "<tt>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i32, tt: Json<TransactionTypeForm>, user: User) -> Status {
    debug!("UPDATE_TRANSACTION_TYPE_REQUEST");
    if TransactionType::update(id, &tt.into_inner(), &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to update transaction_type but an error occurred!");
        Status::InternalServerError
    }
}

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i32, user: User) -> Status {
    debug!("DELETE_TRANSACTION_TYPE_REQUEST");
    if TransactionType::delete(id, &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to delete transaction_type but an error occurred!");
        Status::InternalServerError
    }
}

///
///
pub fn get_mount() -> Vec<rocket::Route> {
    routes![read, read_one]
}