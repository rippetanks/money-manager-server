
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::Cookies;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::place::model::{Place, PlaceForm};
use crate::user::model::User;
use diesel::result::Error;

pub mod model;

#[post("/", data = "<place_json>", format = "application/json")]
fn create(conn: MoneyManagerDB, place_json: Json<PlaceForm>, user: User) -> Result<Json<Place>, Status> {
    debug!("CREATE_PLACE_REQUEST");
    let mut place = place_json.into_inner();
    place.id_user = Some(user.id);
    Place::create(place, &conn)
        .map(|p| {
            info!("place create successfully {}", p.id);
            Json(p)
        })
        .map_err(|e| {
            error!("Can not create place caused by {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Place>, Status> {
    debug!("READ_ONE_PLACE_REQUEST");
    let result = Place::read_by_id(id, &conn);
    if result.is_err() {
        warn!("The user attempts to access transaction that maybe does not exist! {}", result.err().unwrap());
        Err(Status::NotFound)
    } else if check_place_property(&result.as_ref(), &user) {
        Ok(Json(result.ok().unwrap()))
    } else {
        Err(Status::Forbidden)
    }
}

#[get("/user")]
pub fn read_by_user(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Place>>, Custom<String>> {
    debug!("READ_BY_USER_PLACE_REQUEST");
    let result = Place::read_by_user(user.id, &conn);
    Place::unpack(result)
}

#[put("/<id>", data = "<place>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, place: Json<PlaceForm>, user: User) -> Status {
    debug!("UPDATE_PLACE_REQUEST");
    let p = Place::read_by_id(id, &conn);
    if p.is_err() {
        warn!("The user attempts to access place that does not exist! {}", p.err().unwrap());
        return Status::NotFound
    }
    // check if place can be updated
    if check_place_property(&p.as_ref(), &user) {
        if Place::update(id, &place.into_inner(), &conn) {
            Status::NoContent
        } else {
            warn!("The user attempts to update place but an error occurred!");
            Status::InternalServerError
        }
    } else {
        Status::Forbidden
    }
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Status {
    debug!("DELETE_PLACE_REQUEST");
    let p = Place::read_by_id(id, &conn);
    if p.is_err() {
        warn!("The user attempts to access place that does not exist! {}", p.err().unwrap());
        return Status::NotFound
    }
    // check if place can be deleted
    if check_place_property(&p.as_ref(), &user) {
        if Place::delete(id, &conn) {
            Status::NoContent
        } else {
            warn!("The user attempts to delete place but an error occurred!");
            Status::InternalServerError
        }
    } else {
        Status::Forbidden
    }
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/place", routes![read_one, read_by_user, create, update, delete])
}

fn check_place_property(place: &Result<&Place, &Error>, user: &User) -> bool {
    let p = place.unwrap();
    if p.id_user.is_some() && p.id_user.unwrap() != user.id {
        warn!("The user attempts to access place that does not belong to it!");
        return false
    }
    true
}
