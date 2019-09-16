
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::detail::model::{Detail, DetailForm};
use crate::user::model::User;

pub mod model;

#[derive(Debug,Deserialize)]
struct DetailJSON<'a> {
    pub description: &'a str
}

#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<DetailJSON>, user: User) -> Result<Json<Detail>, Status> {
    debug!("CREATE_DETAIL_REQUEST");
    let detail = DetailForm {
        description: json.description,
        id_user: Some(user.id)
    };
    Detail::create(&detail, &conn)
        .map(|result| {
            info!("result create successfully {}", result.id);
            Json(result)
        })
        .map_err(|e| {
            error!("Can not create detail: {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Detail>, Status> {
    debug!("READ_ONE_DETAIL_REQUEST");
    let detail = get_by_id(id, &conn)?;
    check_property(&detail, &user)?;
    Ok(Json(detail))
}

#[get("/user")]
pub fn read_by_user(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Detail>>, Custom<String>> {
    debug!("READ_BY_USER_DETAIL_REQUEST");
    let result = Detail::read_by_user(&user, &conn);
    Detail::unpack(result)
}

#[put("/<id>", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, json: Json<DetailJSON>, user: User) -> Result<Status, Status> {
    debug!("UPDATE_DETAIL_REQUEST");
    let form = DetailForm {
        description: json.description,
        id_user: Some(user.id)
    };
    let detail = get_by_id(id, &conn)?;
    check_property(&detail, &user)?;
    let update = Detail::update(&detail, &form, &conn);
    Detail::finalize_update_delete(update)
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Result<Status, Status> {
    debug!("DELETE_DETAIL_REQUEST");
    let detail = get_by_id(id, &conn)?;
    check_property(&detail, &user)?;
    let delete = Detail::delete(&detail, &conn);
    Detail::finalize_update_delete(delete)
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/detail", routes![read_one, read_by_user, create, update, delete])
}

///
///
pub fn get_and_check(id_detail: i64, user: &User, conn: &MoneyManagerDB) -> Result<Detail, Status> {
    let detail = get_by_id(id_detail, conn)?;
    check_property(&detail, user)?;
    Ok(detail)
}

// #################################################################################################

fn get_by_id(id: i64, conn: &MoneyManagerDB) -> Result<Detail, Status> {
    Detail::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read detail: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

fn check_property(detail: &Detail, user: &User) -> Result<(), Status> {
    if detail.id_user.is_some() && detail.id_user.unwrap() != user.id {
        warn!("The user attempts to access detail that does not belong to it!");
        Err(Status::Forbidden)
    } else {
        Ok(())
    }
}
