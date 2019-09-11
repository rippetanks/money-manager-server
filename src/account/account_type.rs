
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::account::model::{AccountType, AccountTypeForm};
use crate::user::model::User;

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[post("/", data = "<at>", format = "application/json")]
fn create(conn: MoneyManagerDB, at: Json<AccountTypeForm>, user: User) -> Result<Json<AccountType>, Status> {
    debug!("CREATE_ACCOUNT_TYPE_REQUEST");
    AccountType::create(at.into_inner(), &conn)
        .map(|a| {
            info!("account_type create successfully {}", a.id);
            Json(a)
        })
        .map_err(|e| {
            error!("Can not create account_type caused by {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i32, user: User) -> Result<Json<AccountType>, Status> {
    debug!("READ_ONE_ACCOUNT_TYPE_REQUEST");
    AccountType::read_by_id(id, &conn)
        .map(Json)
        .map_err(|e| {
            error!("Can not read account_type by {}", e);
            Status::NotFound
        })
}

#[get("/")]
fn read(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<AccountType>>, Custom<String>> {
    debug!("READ_ALL_ACCOUNT_TYPE_REQUEST");
    let result = AccountType::read(&conn);
    AccountType::unpack(result)
}

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[put("/<id>", data = "<at>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i32, at: Json<AccountTypeForm>, user: User) -> Status {
    debug!("UPDATE_ACCOUNT_TYPE_REQUEST");
    if AccountType::update(id, &at.into_inner(), &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to update account_type but an error occurred!");
        Status::InternalServerError
    }
}

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i32, user: User) -> Status {
    debug!("DELETE_ACCOUNT_TYPE_REQUEST");
    if AccountType::delete(id, &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to delete account_type but an error occurred!");
        Status::InternalServerError
    }
}

///
///
pub fn get_mount() -> Vec<rocket::Route> {
    routes![read, read_one]
}