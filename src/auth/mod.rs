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

use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::State;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::auth::model::Auth;
use crate::user::model::User;
use crate::controller::Extras;

pub mod model;
pub mod auth;

#[derive(Debug,Deserialize)]
struct AuthJSON<'a> {
    email: &'a str,
    password: &'a str
}

#[post("/<id>", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, id: i64, json: Json<AuthJSON>) -> Result<Status, Status> {
    debug!("CREATE_AUTH_REQUEST");
    let auth = auth::create_auth(json.email, json.password, None, id).unwrap();
    match Auth::create(&auth, &conn) {
        Ok(_) => {
            info!("auth create successfully for user {}!", id);
            Ok(Status::NoContent)
        },
        Err(e) => {
            error!("Can not create auth for user {} caused by {}", id, e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/login", data = "<json>", format = "application/json")]
fn login(conn: MoneyManagerDB, json: Json<AuthJSON>, extra: State<Extras>) -> Result<String, Status> {
    debug!("LOGIN_REQUEST");
    let auth = Auth::read_by_email(json.email, &conn);
    match auth {
        Ok(auth) if auth::login(&auth, json.password, &conn) => {
            finalize_login(&auth, &json, &conn, &extra)
        },
        Ok(_) => {
            warn!("Wrong credential! Can not login the user: {}", json.email);
            Err(Status::Unauthorized)
        },
        Err(e) if e.eq(&Error::NotFound) => {
            error!("Can not login the user {}: {}", json.email, e);
            Err(Status::NotFound)
        },
        Err(e) => {
            error!("Can not login the user {}: {}", json.email, e);
            Err(Status::InternalServerError)
        }
    }
}

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[get("/")]
fn read(conn: MoneyManagerDB, _user: User) -> Result<Json<Vec<Auth>>, Custom<String>> {
    warn!("READ_AUTH - DISABLED FOR SECURITY REASON");
    let result = Auth::read(&conn);
    Auth::unpack(result)
}

#[get("/")]
fn read_one(conn: MoneyManagerDB, user: User) -> Result<Json<Auth>, Status> {
    debug!("READ_AUTH_BY_USER_REQUEST");
    let result = Auth::read_by_user(&user, &conn);
    match result {
        Ok(mut result) => {
            info!("The user {} has accessed his authentication data!", result.id);
            // for security reasons, passwords and related information are not sent
            Auth::mask(&mut result);
            Ok(Json(result))
        },
        Err(e) if e.eq(&Error::NotFound) => {
            warn!("auth not found for user {}: {}", user.id, e);
            Err(Status::NotFound)
        },
        Err(e) => {
            error!("Can not get auth for user {}: {}", user.id, e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, user: User, json: Json<AuthJSON>) -> Status {
    debug!("UPDATE_AUTH_REQUEST");
    let update = auth::create_auth(json.email, json.password, None, user.id)
        .map_err(|()| error!("Can not create auth!") ).unwrap();
    let result = Auth::update(user.id, &update, &conn);
    match result {
        Ok(n) if n > 0 => {
            info!("The user {} has updated his authentication data!", user.id);
            Status::NoContent
        },
        Ok(_) => {
            warn!("Auth data not found!");
            Status::NotFound
        },
        Err(e) => {
            error!("Can not update auth for user {}: {}", user.id, e);
            Status::InternalServerError
        }
    }
}

#[delete("/")]
fn delete(conn: MoneyManagerDB, user: User) -> Status {
    debug!("DELETE_AUTH_REQUEST");
    let result = Auth::delete(user.id, &conn);
    match result {
        Ok(n) if n > 0 => {
            info!("The user {} has deleted his authentication data!", user.id);
            Status::NoContent
        },
        Ok(_) => {
            warn!("Auth data not found!");
            Status::NotFound
        },
        Err(e) => {
            error!("Can not delete auth for user {}: {}", user.id, e);
            Status::InternalServerError
        }
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/auth", routes![create, login, read_one, update, delete])
}

// #################################################################################################

fn finalize_login(auth: &Auth, json: &Json<AuthJSON>, conn: &MoneyManagerDB, extra: &State<Extras>) -> Result<String, Status> {
    let user = User::read_by_id(auth.id, &conn).map_err(|e| {
        error!("Can not find the user {} caused by {}", json.email, e.to_string());
        Status::NotFound
    })?;
    let token = auth::create_token(&user, &extra);
    match token {
        Ok(t) => {
            info!("The user {} has just logged in!", user.id);
            Ok(json!({"token": t}).to_string())
        },
        Err(s) => {
            error!("Can not login the user: {}", json.email);
            Err(s)
        }
    }
}

