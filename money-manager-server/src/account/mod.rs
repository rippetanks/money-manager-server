
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::account::model::{Account, AccountForm, AccountUser};
use crate::user::model::User;
use log4rs::filter::Response::Accept;

pub mod model;

mod account_type;

#[post("/", data = "<account>", format = "application/json")]
fn create(conn: MoneyManagerDB, account: Json<AccountForm>, user: User) -> Result<Json<Account>, Status> {
    debug!("CREATE_ACCOUNT_REQUEST");
    let a = Account::create(account.into_inner(), &conn);
    match a {
        Ok(a) => {
            let au = AccountUser {
                id_user: user.id,
                id_account: a.id
            };
            if !AccountUser::create(au, &conn) {
                error!("Can not create account: error create account_user!");
                Account::delete(a.id, &conn);
                Err(Status::InternalServerError)
            } else {
                info!("account create successfully {}", a.id);
                Ok(Json(a))
            }
        },
        Err(e) => {
            error!("Can not create account caused by {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Account>, Status> {
    debug!("READ_ONE_ACCOUNT_REQUEST");
    let result = Account::read_by_id(id, &conn);
    match &result {
        Ok(result) => {
            // a user can access his own account
            let au = AccountUser::read_by_au(&conn, result, &user);
            if au.is_err() {
                warn!("The user attempts to access account that does not belong to it! {}",
                      au.err().unwrap());
                return Err(Status::Forbidden);
            }
            Ok(Json((*result).clone()))
        }
        Err(_) => {
            warn!("The user attempts to access account that does not exist!");
            Err(Status::NotFound)
        }
    }
}

#[get("/user")]
pub fn read_by_user(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Account>>, Custom<String>> {
    debug!("READ_BY_USER_ACCOUNT_REQUEST");
    let result = Account::read_by_user(user.id, &conn);
    Account::unpack(result)
}

#[put("/<id>", data = "<account>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, account: Json<AccountForm>, user: User) -> Status {
    debug!("UPDATE_ACCOUNT_REQUEST");
    // check if account can be updated
    let tmp = Account::read_by_id(id, &conn);
    if tmp.is_err() {
        warn!("The user attempts to update account but an error occurred!");
        return Status::InternalServerError;
    }
    let au = AccountUser::read_by_au(&conn, &tmp.ok().unwrap(), &user);
    if au.is_err() {
        warn!("The user attempts to access account that does not belong to it! {}", au.err().unwrap());
        return Status::Forbidden
    }
    if Account::update(id, &account.into_inner(), &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to update account but an error occurred!");
        Status::InternalServerError
    }
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Status {
    debug!("DELETE_ACCOUNT_REQUEST");
    // check if causal can be deleted
    let tmp = Account::read_by_id(id, &conn);
    if tmp.is_err() {
        warn!("The user attempts to delete account but an error occurred! {}", tmp.err().unwrap());
        return Status::InternalServerError;
    }
    let au = AccountUser::read_by_au(&conn, &tmp.ok().unwrap(), &user);
    if au.is_err() {
        warn!("The user attempts to access account that does not belong to it! {}", au.err().unwrap());
        return Status::Forbidden
    }
    if Account::delete(id, &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to delete account but an error occurred!");
        Status::InternalServerError
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/account", routes![read_one, read_by_user, create, update, delete])
}

///
///
pub fn mount_account_type(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/account/type", account_type::get_mount())
}
