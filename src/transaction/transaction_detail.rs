/*
    Copyright (C) 2019  Simone Martelli

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::transaction::model::{Transaction, TransactionDetail};
use crate::detail::model::Detail;
use crate::user::model::User;
use crate::transaction;
use crate::detail;

#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<TransactionDetail>, user: User) -> Result<Status, Status> {
    debug!("CREATE_TRANSACTION_DETAIL_REQUEST");
    let form = json.into_inner();
    transaction::get_and_check(form.id_transaction, &user, &conn)?;
    detail::get_and_check(form.id_detail, &user, &conn)?;
    if TransactionDetail::create(&form, &conn) {
        info!("transaction detail create successfully");
        Ok(Status::NoContent)
    } else {
        error!("Can not create transaction detail");
        Err(Status::InternalServerError)
    }
}

#[get("/transaction/<id>")]
fn read_by_transaction(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<TransactionDetail>>, Custom<String>> {
    debug!("READ_BY_TRANSACTION_TRANSACTION_DETAIL_REQUEST");
    let transaction = transaction::get_and_check(id, &user, &conn)
        .map_err(|s| Custom(s, String::new()))?;
    let result = TransactionDetail::read_by_transaction(&conn, &transaction);
    TransactionDetail::unpack(result)
}

#[get("/detail/<id>")]
pub fn read_by_detail(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<TransactionDetail>>, Custom<String>> {
    debug!("READ_BY_DETAIL_TRANSACTION_DETAIL_REQUEST");
    let detail = detail::get_and_check(id, &user, &conn)
        .map_err(|s| Custom(s, String::new()))?;
    let result = TransactionDetail::read_by_detail(&conn, &detail);
    TransactionDetail::unpack(result)
}

#[put("/", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, json: Json<TransactionDetail>, user: User) -> Result<Status, Status> {
    debug!("UPDATE_TRANSACTION_DETAIL_REQUEST");
    let form = json.into_inner();
    let transaction = transaction::get_and_check(form.id_transaction, &user, &conn)?;
    let detail = detail::get_and_check(form.id_detail, &user, &conn)?;
    let td = get_by_td(&transaction, &detail, &conn)?;
    let result = TransactionDetail::update(&td, &conn);
    TransactionDetail::finalize_update_delete(result)
}

#[delete("/transaction/<id_transaction>/detail/<id_detail>")]
fn delete(conn: MoneyManagerDB, id_transaction: i64, id_detail: i64, user: User) -> Result<Status, Status> {
    debug!("DELETE_TRANSACTION_DETAIL_REQUEST");
    let transaction = transaction::get_and_check(id_transaction, &user, &conn)?;
    let detail = detail::get_and_check(id_detail, &user, &conn)?;
    // let td = get_by_td(&transaction, &detail, &conn)?;
    let result = TransactionDetail::delete_by_td(&conn, &transaction, &detail);
    TransactionDetail::finalize_update_delete(result)
}

///
///
pub fn get_mount() -> Vec<rocket::Route> {
    routes![create, read_by_detail, read_by_transaction, update, delete]
}

// #################################################################################################

fn get_by_td(transaction: &Transaction, detail: &Detail, conn: &MoneyManagerDB) -> Result<TransactionDetail, Status> {
    TransactionDetail::read_by_td(&conn, detail, transaction)
        .map_err(|e| {
            error!("Can not read transaction detail: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}
