
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use chrono::NaiveDate;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::user::model::{User, UserForm};

pub mod model;

#[derive(Debug,Deserialize)]
struct UserJSON<'a> {
    pub name: &'a str,
    pub surname: &'a str,
    pub phone: Option<&'a str>,
    pub country: Option<&'a str>,
    pub address: Option<&'a str>,
    pub birthdate: Option<NaiveDate>,
    pub note: Option<&'a str>
}

#[post("/", data = "<user>", format = "application/json")]
fn create(conn: MoneyManagerDB, user: Json<UserJSON>) -> Result<Json<User>, Status> {
    debug!("CREATE_USER_REQUEST");
    let insert = UserForm {
        name: user.name.to_string(),
        surname: user.surname.to_string(),
        phone: user.phone.map(|s| s.to_string()),
        country: user.country.map(|c| c.to_string()),
        address: user.address.map(|a| a.to_string()),
        birthdate: user.birthdate,
        note: user.note.map(|n| n.to_string())
    };
    User::create(&insert, &conn)
        .map(|u| {
            info!("user create successfully {}", u.id);
            Json(u)
        })
        .map_err(|e| {
            error!("Can not create user caused by {}", e);
            Status::InternalServerError
        })
}

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[get("/")]
fn read(conn: MoneyManagerDB, _user: User) -> Result<Json<Vec<User>>, Custom<String>> {
    debug!("READ_USER_REQUEST");
    let result = User::read(&conn);
    User::unpack(result)
}

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, _user: User) -> Option<Json<User>> {
    debug!("READ_ONE_USER_REQUEST");
    let result = User::read_by_id(id, &conn);
    match result {
        Ok(result) => Some(Json(result)),
        Err(e) => {
            warn!("The user attempts to access user data that does not exist! {}", e);
            None
        }
    }
}

#[get("/user")]
fn read_for_user(user: User) -> Json<User> {
    debug!("READ_FOR_USER_REQUEST");
    info!("The user {} has accessed his user private data!", user.id);
    Json(user)
}

#[put("/user", data = "<user_json>")]
fn update_for_user(conn: MoneyManagerDB, user_json: Json<UserJSON>, user: User) -> Status {
    debug!("UPDATE_FOR_USER_REQUEST");
    let update = UserForm {
        name: user_json.name.to_string(),
        surname: user_json.surname.to_string(),
        phone: user_json.phone.map(|s| s.to_string()),
        country: user_json.country.map(|c| c.to_string()),
        address: user_json.address.map(|a| a.to_string()),
        birthdate: user_json.birthdate,
        note: user_json.note.map(|n| n.to_string())
    };
    if User::update(user.id, &update, &conn) {
        info!("The user {} has updated his user private data!", user.id);
        Status::NoContent
    } else {
        warn!("The user attempts to update user private data but an error occurred!");
        Status::InternalServerError
    }
}

#[delete("/user")]
fn delete_for_user(conn: MoneyManagerDB, user: User) -> Status {
    debug!("DELETE_FOR_USER_REQUEST");
    if User::delete(user.id, &conn) {
        info!("The user {} has deleted his user private data!", user.id);
        Status::NoContent
    } else {
        warn!("The user attempts to delete user private data but an error occurred!");
        Status::InternalServerError
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/user", routes![read_for_user, create, update_for_user, delete_for_user])
}
