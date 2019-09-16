
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::transaction::model::{Transaction, TransactionForm};
use crate::account;
use crate::user::model::User;

pub mod model;

mod transaction_type;
mod transaction_detail;

#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<TransactionForm>, user: User) -> Result<Json<Transaction>, Status> {
    debug!("CREATE_TRANSACTION_REQUEST");
    let form = json.into_inner();
    account::check(form.id_account, &user, &conn)?;
    Transaction::create(&form, &conn)
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
    let transaction = get_by_id(id, &conn)?;
    // user can access his own transaction
    account::check(transaction.id_account, &user, &conn)?;
    Ok(Json(transaction))
}

#[get("/account/<id>")]
pub fn read_by_account(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<Transaction>>, Custom<String>> {
    debug!("READ_BY_ACCOUNT_TRANSACTION_REQUEST");
    let account = account::get_and_check(id, &user, &conn)
        .map_err(|s| Custom(s, String::new()))?;
    let result = Transaction::read_by_account(&account, &conn);
    Transaction::unpack(result)
}

#[put("/<id>", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, json: Json<TransactionForm>, user: User) -> Result<Status, Status> {
    debug!("UPDATE_TRANSACTION_REQUEST");
    let transaction = get_by_id(id, &conn)?;
    // check if transaction can be updated
    check_property(&transaction, &user, &conn)?;
    let result = Transaction::update(&transaction, &json.into_inner(), &conn);
    Transaction::finalize_update_delete(result)
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Result<Status, Status> {
    debug!("DELETE_TRANSACTION_REQUEST");
    let transaction = get_by_id(id, &conn)?;
    // check if causal can be deleted
    check_property(&transaction, &user, &conn)?;
    let result = Transaction::delete(&transaction, &conn);
    Transaction::finalize_update_delete(result)
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

///
///
pub fn get_and_check(id_transaction: i64, user: &User, conn: &MoneyManagerDB) -> Result<Transaction, Status> {
    let transaction = get_by_id(id_transaction, conn)?;
    check_property(&transaction, user, conn)?;
    Ok(transaction)
}

// #################################################################################################

fn get_by_id(id: i64, conn: &MoneyManagerDB) -> Result<Transaction, Status> {
    Transaction::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read transaction: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

fn check_property(transaction: &Transaction, user: &User, conn: &MoneyManagerDB) -> Result<(), Status> {
    let c = account::check(transaction.id_account, user, conn);
    if c.is_err() {
        warn!("The user attempts to access transaction that does not belong to it!");
        Err(Status::Forbidden)
    } else {
        Ok(())
    }
}
