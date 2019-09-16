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
use crate::giro::model::{Giro, GiroForm};
use crate::account;
use crate::user::model::User;

pub mod model;

#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<GiroForm>, user: User) -> Result<Json<Giro>, Status> {
    debug!("CREATE_GIRO_REQUEST");
    let form = json.into_inner();
    check_source_id_property(form.id_source_account, &user, &conn)?;
    Giro::create(&form, &conn)
        .map(|giro| {
            info!("giro create successfully {}", giro.id);
            Json(giro)
        })
        .map_err(|e| {
            error!("Can not create giro caused by {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Giro>, Status> {
    debug!("READ_ONE_GIRO_REQUEST");
    let giro = get_by_id(id, &conn)?;
    // a user can access his own giro
    check_source_property(&giro, &user, &conn)?;
    check_destination_property(&giro, &user, &conn)?;
    Ok(Json(giro))
}

#[get("/account/source/<id>")]
fn read_by_source(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<Giro>>, Custom<String>> {
    debug!("READ_BY_ACCOUNT_SOURCE_GIRO_REQUEST");
    let account = account::get_and_check(id, &user, &conn)
        .map_err(|s| Custom(s, String::new()))?;
    let result = Giro::read_by_source(&account, &conn);
    Giro::unpack(result)
}

#[get("/account/destination/<id>")]
fn read_by_destination(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Vec<Giro>>, Custom<String>> {
    debug!("READ_BY_ACCOUNT_DESTINATION_GIRO_REQUEST");
    let account = account::get_and_check(id, &user, &conn)
        .map_err(|s| Custom(s, String::new()))?;
    let result = Giro::read_by_destination(&account, &conn);
    Giro::unpack(result)
}

#[put("/<id>", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, json: Json<GiroForm>, user: User) -> Result<Status, Status> {
    debug!("UPDATE_GIRO_REQUEST");
    let giro = get_by_id(id, &conn)?;
    // check if account can be updated
    check_source_property(&giro, &user, &conn)?;
    let result = Giro::update(&giro, &json.into_inner(), &conn);
    Giro::finalize_update_delete(result)
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Result<Status, Status> {
    debug!("DELETE_GIRO_REQUEST");
    let giro = get_by_id(id, &conn)?;
    // check if causal can be deleted
    check_source_property(&giro, &user, &conn)?;
    let result = Giro::delete(&giro, &conn);
    Giro::finalize_update_delete(result)
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/giro", routes![read_one, read_by_source, read_by_destination, create, update, delete])
}

// #################################################################################################

fn get_by_id(id: i64, conn: &MoneyManagerDB) -> Result<Giro, Status> {
    Giro::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read giro: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

fn check_source_property(giro: &Giro, user: &User, conn: &MoneyManagerDB) -> Result<(), Status> {
    check_source_id_property(giro.id_source_account, user, conn)
}

fn check_source_id_property(id_account: i64, user: &User, conn: &MoneyManagerDB) -> Result<(), Status> {
    let c = account::check(id_account, user, conn);
    if c.is_err() {
        warn!("The user attempts to access giro (source account) that does not belong to it!");
        Err(Status::Forbidden)
    } else {
        Ok(())
    }
}

fn check_destination_property(giro: &Giro, user: &User, conn: &MoneyManagerDB) -> Result<(), Status> {
    let c = account::check(giro.id_destination_account, user, conn);
    if c.is_err() {
        warn!("The user attempts to access giro (destination account) that does not belong to it!");
        Err(Status::Forbidden)
    } else {
        Ok(())
    }
}
