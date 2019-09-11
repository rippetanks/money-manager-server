
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::detail::model::{Detail, DetailForm};
use crate::user::model::User;

pub mod model;

#[post("/", data = "<detail_json>", format = "application/json")]
fn create(conn: MoneyManagerDB, detail_json: Json<DetailForm>, user: User) -> Result<Json<Detail>, Status> {
    debug!("CREATE_DETAIL_REQUEST");
    let mut detail = detail_json.into_inner();
    detail.id_user = Some(user.id);
    Detail::create(detail, &conn)
        .map(|d| {
            info!("detail create successfully {}", d.id);
            Json(d)
        })
        .map_err(|e| {
            error!("Can not create detail caused by {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Detail>, Status> {
    debug!("READ_ONE_DETAIL_REQUEST");
    let result = Detail::read_by_id(id, &conn);
    if result.is_err() {
        warn!("The user attempts to access detail that maybe does not exist! {}", result.err().unwrap());
        Err(Status::NotFound)
    } else if check_detail_property(&result.as_ref(), &user) {
        Ok(Json(result.ok().unwrap()))
    } else {
        Err(Status::Forbidden)
    }
}

#[get("/user")]
pub fn read_by_user(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Detail>>, Custom<String>> {
    debug!("READ_BY_USER_DETAIL_REQUEST");
    let result = Detail::read_by_user(user.id, &conn);
    Detail::unpack(result)
}

#[put("/<id>", data = "<detail>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, detail: Json<DetailForm>, user: User) -> Status {
    debug!("UPDATE_DETAIL_REQUEST");
    let p = Detail::read_by_id(id, &conn);
    if p.is_err() {
        warn!("The user attempts to access detail that does not exist! {}", p.err().unwrap());
        return Status::NotFound
    }
    // check if detail can be updated
    if check_detail_property(&p.as_ref(), &user) {
        if Detail::update(id, &detail.into_inner(), &conn) {
            Status::NoContent
        } else {
            warn!("The user attempts to update detail but an error occurred!");
            Status::InternalServerError
        }
    } else {
        Status::Forbidden
    }
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Status {
    debug!("DELETE_DETAIL_REQUEST");
    let d = Detail::read_by_id(id, &conn);
    if d.is_err() {
        warn!("The user attempts to access detail that does not exist! {}", d.err().unwrap());
        return Status::NotFound
    }
    // check if place can be deleted
    if check_detail_property(&d.as_ref(), &user) {
        if Detail::delete(id, &conn) {
            Status::NoContent
        } else {
            warn!("The user attempts to delete detail but an error occurred!");
            Status::InternalServerError
        }
    } else {
        Status::Forbidden
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/detail", routes![read_one, read_by_user, create, update, delete])
}

fn check_detail_property(detail: &Result<&Detail, &Error>, user: &User) -> bool {
    let d = detail.unwrap();
    if d.id_user.is_some() && d.id_user.unwrap() != user.id {
        warn!("The user attempts to access place that does not belong to it!");
        return false
    }
    true
}
