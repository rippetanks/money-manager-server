
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::transaction::model::{TransactionType, TransactionTypeForm};
use crate::user::model::User;

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<TransactionTypeForm>, _user: User) -> Result<Json<TransactionType>, Status> {
    debug!("CREATE_TRANSACTION_TYPE_REQUEST");
    TransactionType::create(&json.into_inner(), &conn)
        .map(|tt| {
            info!("transaction_type create successfully {}", tt.id);
            Json(tt)
        })
        .map_err(|e| {
            error!("Can not create transaction_type caused by {}", e);
            Status::InternalServerError
        })
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i32, _user: User) -> Result<Json<TransactionType>, Status> {
    debug!("READ_ONE_TRANSACTION_TYPE_REQUEST");
    get_by_id(id, &conn).map(Json)
}

#[get("/")]
fn read(conn: MoneyManagerDB, _user: User) -> Result<Json<Vec<TransactionType>>, Custom<String>> {
    debug!("READ_ALL_TRANSACTION_TYPE_REQUEST");
    let result = TransactionType::read(&conn);
    TransactionType::unpack(result)
}

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[put("/<id>", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i32, json: Json<TransactionTypeForm>, _user: User) -> Result<Status, Status> {
    debug!("UPDATE_TRANSACTION_TYPE_REQUEST");
    let tt = get_by_id(id, &conn)?;
    let result = TransactionType::update(&tt, &json.into_inner(), &conn);
    TransactionType::finalize_update_delete(result)
}

/*  DISABLED FOR SECURITY REASON */
#[allow(dead_code)]
#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i32, _user: User) -> Result<Status, Status> {
    debug!("DELETE_TRANSACTION_TYPE_REQUEST");
    let tt = get_by_id(id, &conn)?;
    let result = TransactionType::delete(&tt, &conn);
    TransactionType::finalize_update_delete(result)
}

///
///
pub fn get_mount() -> Vec<rocket::Route> {
    routes![read, read_one]
}

// #################################################################################################

fn get_by_id(id: i32, conn: &MoneyManagerDB) -> Result<TransactionType, Status> {
    TransactionType::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read transaction_type: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}
