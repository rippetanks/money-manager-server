
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::transaction::model::{Transaction, TransactionDetail};
use crate::detail::model::Detail;
use crate::user::model::User;

#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<TransactionDetail>, user: User) -> Status {
    debug!("CREATE_TRANSACTION_DETAIL_REQUEST");
    if TransactionDetail::create(json.into_inner(), &conn) {
        info!("transaction detail create successfully");
        Status::NoContent
    } else {
        error!("Can not create transaction detail!");
        Status::InternalServerError
    }
}

#[get("/transaction/<id>")]
fn read_by_transaction(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<TransactionDetail>>, Custom<String>> {
    debug!("READ_BY_TRANSACTION_TRANSACTION_DETAIL_REQUEST");
    // TODO se non ho valori devo ritornare 204 e non 404 (forse anche in altre situazioni)
    let transaction = Transaction::read_by_id(id, &conn);
    if transaction.is_err() {
        Err(Custom(Status::NotFound, String::new()))
    } else {
        let result = TransactionDetail::read_by_transaction(&conn, &transaction.ok().unwrap());
        TransactionDetail::unpack(result)
    }
}

#[get("/detail/<id>")]
pub fn read_by_detail(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<TransactionDetail>>, Custom<String>> {
    debug!("READ_BY_DETAIL_TRANSACTION_DETAIL_REQUEST");
    let detail = Detail::read_by_id(id, &conn);
    if detail.is_err() {
        Err(Custom(Status::NotFound, String::new()))
    } else {
        let result = TransactionDetail::read_by_detail(&conn, &detail.ok().unwrap());
        TransactionDetail::unpack(result)
    }
}

#[put("/", data = "<td>", format = "application/json")]
fn update(conn: MoneyManagerDB, td: Json<TransactionDetail>, user: User) -> Status {
    debug!("UPDATE_TRANSACTION_DETAIL_REQUEST");
    if TransactionDetail::update(&td.into_inner(), &conn) {
        Status::NoContent
    } else {
        warn!("The user attempts to update transaction detail but an error occurred!");
        Status::InternalServerError
    }
}

#[delete("/transaction/<id_transaction>/detail/<id_detail>")]
fn delete(conn: MoneyManagerDB, id_transaction: i64, id_detail: i64, user: User) -> Status {
    debug!("DELETE_TRANSACTION_DETAIL_REQUEST");
    let td = TransactionDetail::read_by_td(&conn, id_detail, id_transaction);
    if td.is_err() {
        warn!("The user attempts to delete detail but an error occurred! {}", td.err().unwrap());
        return Status::NotFound
    }
    if TransactionDetail::delete_by_td(&conn, &td.ok().unwrap()) {
        Status::NoContent
    } else {
        warn!("The user attempts to delete transaction detail but an error occurred!");
        Status::InternalServerError
    }
}

///
///
pub fn get_mount() -> Vec<rocket::Route> {
    routes![create, read_by_detail, read_by_transaction, update, delete]
}
