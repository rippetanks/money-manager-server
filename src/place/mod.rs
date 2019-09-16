
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::place::model::{Place, PlaceForm};
use crate::user::model::User;

pub mod model;

#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<PlaceForm>, user: User) -> Result<Json<Place>, Status> {
    debug!("CREATE_PLACE_REQUEST");
    let mut place = json.into_inner();
    place.id_user = Some(user.id);
    Place::create(&place, &conn)
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
    let place = get_by_id(id, &conn)?;
    // a user can access his own place or the default ones
    if place.id_user.is_some() {
        check_property(&place, &user)?;
    }
    Ok(Json(place))
}

#[get("/user")]
pub fn read_by_user(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Place>>, Custom<String>> {
    debug!("READ_BY_USER_PLACE_REQUEST");
    let result = Place::read_by_user(&user, &conn);
    Place::unpack(result)
}

#[put("/<id>", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, json: Json<PlaceForm>, user: User) -> Result<Status, Status> {
    debug!("UPDATE_PLACE_REQUEST");
    let mut place = get_by_id(id, &conn)?;
    place.id_user = Some(user.id);
    // check if place can be updated
    check_property(&place, &user)?;
    let result = Place::update(&place, &json.into_inner(), &conn);
    Place::finalize_update_delete(result)
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Result<Status, Status> {
    debug!("DELETE_PLACE_REQUEST");
    let place = get_by_id(id, &conn)?;
    // check if place can be deleted
    check_property(&place, &user)?;
    let result = Place::delete(&place, &conn);
    Place::finalize_update_delete(result)
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/place", routes![read_one, read_by_user, create, update, delete])
}

// #################################################################################################

fn get_by_id(id: i64, conn: &MoneyManagerDB) -> Result<Place, Status> {
    Place::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read place: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

fn check_property(place: &Place, user: &User) -> Result<(), Status> {
    if place.id_user.is_none() || place.id_user.unwrap() != user.id {
        warn!("The user attempts to access place that does not belong to it!");
        Err(Status::Forbidden)
    } else {
        Ok(())
    }
}
