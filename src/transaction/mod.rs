
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::transaction::model::{Transaction, TransactionForm, TransactionType, TransactionTypeForm};
use crate::account::model::AccountUser;
use crate::user::model::User;
use std::intrinsics::transmute;

pub mod model;

mod transaction_type;
mod transaction_detail;

#[post("/", data = "<transaction>", format = "application/json")]
fn create(conn: MoneyManagerDB, transaction: Json<TransactionForm>, user: User) -> Result<Json<Transaction>, Status> {
    debug!("CREATE_TRANSACTION_REQUEST");
    Transaction::create(transaction.into_inner(), &conn)
        .map(|t| {
            info!("transaction create successfully {}", t.id);
            Json(t)
        })
        .map_err(|e| {
           error!("Can not create transaction caused by {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Transaction>, Status> {
    debug!("READ_ONE_TRANSACTION_REQUEST");
    let result = Transaction::read_by_id(id, &conn);
    if result.is_err() {
        warn!("The user attempts to access transaction that maybe does not exist! {}", result.err().unwrap());
        return Err(Status::NotFound)
    }
    let transaction = result.unwrap();
    if check_account_property(transaction.id_account, &conn, &user) {
        Ok(Json(transaction))
    } else {
        Err(Status::Forbidden)
    }
}

#[get("/account/<id>")]
pub fn read_by_account(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<Transaction>>, Custom<String>> {
    debug!("READ_BY_ACCOUNT_TRANSACTION_REQUEST");
    if check_account_property(id, &conn, &user) {
        let result = Transaction::read_by_account(id, &conn);
        Transaction::unpack(result)
    } else {
        Err(Custom(Status::Forbidden, String::new()))
    }
}

#[put("/<id>", data = "<transaction>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, transaction: Json<TransactionForm>, user: User) -> Status {
    debug!("UPDATE_TRANSACTION_REQUEST");
    // check if transaction can be updated
    let t = Transaction::read_by_id(id, &conn);
    if t.is_err() {
        warn!("The user attempts to access transaction that maybe does not exist! {}", t.err().unwrap());
        return Status::NotFound
    }
    if check_account_property(t.unwrap().id_account, &conn, &user) {
        if Transaction::update(id, &transaction.into_inner(), &conn) {
            Status::NoContent
        } else {
            warn!("The user attempts to update transaction but an error occurred!");
            Status::InternalServerError
        }
    } else {
        Status::Forbidden
    }
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Status {
    debug!("DELETE_TRANSACTION_REQUEST");
    // check if causal can be deleted
    let t = Transaction::read_by_id(id, &conn);
    if t.is_err() {
        warn!("The user attempts to access transaction that maybe does not exist! {}", t.err().unwrap());
        return Status::NotFound
    }
    if check_account_property(t.unwrap().id_account, &conn, &user) {
        if Transaction::delete(id, &conn) {
            Status::NoContent
        } else {
            warn!("The user attempts to delete transaction but an error occurred!");
            Status::InternalServerError
        }
    } else {
        Status::Forbidden
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/transaction", routes![read_one, read_by_account, create, update, delete])
}

///
///
pub fn mount_transaction_type(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/transaction/type", transaction_type::get_mount())
}

///
///
pub fn mount_transaction_detail(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/transaction/detail", transaction_detail::get_mount())
}

fn check_account_property(id: i64, conn: &MoneyManagerDB, user: &User) -> bool {
    let au = AccountUser::read_by_au(conn, user, id);
    if au.is_err() {
        error!("The user attempts to access some data of transaction that maybe does not belong to it! {}", au.err().unwrap());
        return false
    }
    true
}