
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::http::Cookies;
use rocket::State;
use serde::{Serialize, Deserialize};

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::auth::model::Auth;
use crate::user::model::User;
use crate::controller::Extras;
use rocket::response::status::Custom;

pub mod model;
pub mod auth;

#[derive(Debug,Serialize,Deserialize)]
struct AuthCreate {
    email: String,
    password: String
}

#[post("/<id>", data = "<auth_json>", format = "application/json")]
fn create(conn: MoneyManagerDB, id: i64, auth_json: Json<AuthCreate>) -> Result<Status, Status> {
    debug!("CREATE_AUTH_REQUEST");
    let auth = auth::create_auth(&auth_json.email, &auth_json.password, None, id).unwrap();
    match Auth::create(auth, &conn) {
        Ok(_) => {
            info!("auth create successfully for user {}!", id);
            Ok(Status::NoContent)
        },
        Err(e) => {
            error!("Can not create auth for user {} caused by {}", id, e.to_string());
            Err(Status::InternalServerError)
        }
    }
}

#[post("/login", data = "<auth_json>", format = "application/json")]
fn login(conn: MoneyManagerDB, auth_json: Json<AuthCreate>, extra: State<Extras>) -> Result<String, Status> {
    debug!("LOGIN_REQUEST");
    let auth = Auth::read_by_email(&auth_json.email, &conn);
    match &auth {
        Ok(auth) if auth::login(auth, &auth_json.password, &conn) => {
            let user = User::read_by_id(auth.id, &conn)
                .map_err(|e| {
                    error!("Can not find the user {} caused by {}", auth_json.email, e.to_string())
                }).unwrap();
            let token = auth::create_token(&user, &extra);
            match token {
                Ok(t) => {
                    info!("The user {} has just logged in!", user.id);
                    Ok(json!({"token": t}).to_string())
                },
                Err(s) => {
                    error!("Can not login the user {}", auth_json.email);
                    Err(s)
                }
            }
        },
        Ok(_) => {
            error!("Wrong credential! Can not login the user {}", auth_json.email);
            Err(Status::Unauthorized)
        },
        Err(_) => {
            error!("INTERNAL SERVER ERROR");
            Err(Status::InternalServerError)
        }
    }
}

/* DISABLED FOR SECURITY REASON
#[get("/")]
fn read(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Auth>>, Custom<String>> {
    let result = Auth::read(&conn);
    Auth::unpack(result)
}
*/

#[get("/")]
fn read_one(conn: MoneyManagerDB, user: User) -> Option<Json<Auth>> {
    debug!("READ_AUTH_BY_USER_REQUEST");
    let result = Auth::read_by_user(&user, &conn);
    match result {
        Ok(mut result) => {
            info!("The user {} has accessed his authentication data!", result.id);
            // for security reasons, passwords and related information are not sent
            Auth::mask(&mut result);
            Some(Json(result))
        },
        Err(_) => {
            warn!("auth not found for user {}", user.id);
            None
        }
    }
}

#[put("/", data = "<auth_json>", format = "application/json")]
fn update(conn: MoneyManagerDB, user: User, auth_json: Json<AuthCreate>) -> Status {
    debug!("UPDATE_AUTH_REQUEST");
    let update = auth::create_auth(&auth_json.email, &auth_json.password, None, user.id).unwrap();
    if Auth::update(user.id, &update, &conn) {
        info!("The user {} has updated his authentication data!", user.id);
        Status::NoContent
    } else {
        warn!("Can not update auth for user {}", user.id);
        Status::InternalServerError
    }
}

#[delete("/")]
fn delete(conn: MoneyManagerDB, user: User) -> Status {
    debug!("DELETE_AUTH_REQUEST");
    if Auth::delete(user.id, &conn) {
        info!("The user {} has deleted his authentication data!", user.id);
        Status::NoContent
    } else {
        warn!("Can not delete auth for user {}", user.id);
        Status::InternalServerError
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/auth", routes![create, login, read_one, update, delete])
}

