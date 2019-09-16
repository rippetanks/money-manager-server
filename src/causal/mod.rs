
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::causal::model::{Causal, CausalForm};
use crate::user::model::User;

pub mod model;

#[derive(Debug,Deserialize)]
struct CausalJSON<'a> {
    pub description: &'a str
}

#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<CausalJSON>, user: User) -> Result<Json<Causal>, Status> {
    debug!("CREATE_CAUSAL_REQUEST");
    let insert = CausalForm {
        description: json.description,
        id_user: Some(user.id),
    };
    Causal::create(&insert, &conn)
        .map(|result| {
            info!("causal create successfully: {}", result.id);
            Json(result)
        })
        .map_err(|e| {
            error!("Can not create causal: {}", e);
            Status::InternalServerError
        })
}

/* DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[get("/")]
fn read(conn: MoneyManagerDB, _user: User) -> Result<Json<Vec<Causal>>, Custom<String>> {
    debug!("READ_CAUSAL_REQUEST");
    let result = Causal::read(&conn);
    Causal::unpack(result)
}

#[get("/user")]
fn read_for_user(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Causal>>, Custom<String>> {
    debug!("READ_FOR_USER_CAUSAL_REQUEST");
    let result = Causal::read_for_user(&user, &conn);
    Causal::unpack(result)
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Causal>, Status> {
    debug!("READ_ONE_CAUSAL_REQUEST");
    let causal = get_by_id(id, &conn)?;
    // a user can access his own causals or the default ones
    if causal.id_user.is_some() {
        check_property(&causal, &user)?;
    }
    Ok(Json(causal))
}

#[put("/<id>", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, json: Json<CausalJSON>, user: User) -> Result<Status, Status> {
    debug!("UPDATE_CAUSAL_REQUEST");
    let form = CausalForm {
        description: json.description,
        id_user: Some(user.id)
    };
    let causal = get_by_id(id, &conn)?;
    // check if causal can be updated
    check_property(&causal, &user)?;
    let update = Causal::update(&causal, &form, &conn);
    Causal::finalize_update_delete(update)
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Result<Status, Status> {
    debug!("DELETE_CAUSAL_REQUEST");
    let causal = get_by_id(id, &conn)?;
    // check if causal can be deleted
    check_property(&causal, &user)?;
    let delete = Causal::delete(&causal, &conn);
    Causal::finalize_update_delete(delete)
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/causal", routes![read_one, read_for_user, create, update, delete])
}

// #################################################################################################

fn get_by_id(id: i64, conn: &MoneyManagerDB) -> Result<Causal, Status> {
    Causal::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read causal: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

fn check_property(causal: &Causal, user: &User) -> Result<(), Status> {
    if causal.id_user.is_none() || causal.id_user.unwrap() != user.id {
        warn!("The user attempts to access causal that does not belong to it!");
        Err(Status::Forbidden)
    } else {
        Ok(())
    }
}
