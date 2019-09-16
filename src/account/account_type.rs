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

use diesel::result::Error;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::account::model::{AccountType, AccountTypeForm};
use crate::user::model::User;

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<AccountTypeForm>, _user: User) -> Result<Json<AccountType>, Status> {
    debug!("CREATE_ACCOUNT_TYPE_REQUEST");
    AccountType::create(&json.into_inner(), &conn)
        .map(|at| {
            info!("account_type create successfully {}", at.id);
            Json(at)
        })
        .map_err(|e| {
            error!("Can not create account_type caused by {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i32, _user: User) -> Result<Json<AccountType>, Status> {
    debug!("READ_ONE_ACCOUNT_TYPE_REQUEST");
    get_by_id(id, &conn).map(Json)
}

#[get("/")]
fn read(conn: MoneyManagerDB, _user: User) -> Result<Json<Vec<AccountType>>, Custom<String>> {
    debug!("READ_ALL_ACCOUNT_TYPE_REQUEST");
    let result = AccountType::read(&conn);
    AccountType::unpack(result)
}

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[put("/<id>", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i32, json: Json<AccountTypeForm>, _user: User) -> Result<Status, Status> {
    debug!("UPDATE_ACCOUNT_TYPE_REQUEST");
    let at = get_by_id(id, &conn)?;
    let result = AccountType::update(&at, &json.into_inner(), &conn);
    finalize_update_delete(result)
}

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i32, _user: User) -> Result<Status, Status> {
    debug!("DELETE_ACCOUNT_TYPE_REQUEST");
    let at = get_by_id(id, &conn)?;
    let result = AccountType::delete(&at, &conn);
    finalize_update_delete(result)
}

///
///
pub fn get_mount() -> Vec<rocket::Route> {
    routes![read, read_one]
}

// #################################################################################################

fn get_by_id(id: i32, conn: &MoneyManagerDB) -> Result<AccountType, Status> {
    AccountType::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read account_type: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

fn finalize_update_delete(result: Result<usize, Error>) -> Result<Status, Status> {
    match result {
        Ok(n) if n > 0 => Ok(Status::NoContent),
        Ok(_) => {
            warn!("account_type not found!");
            Err(Status::NotFound)
        },
        Err(e) => {
            error!("An error occurred on account_type: {}", e);
            Err(Status::InternalServerError)
        }
    }
}