
use diesel::result::Error;
use diesel::Connection;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;

use crate::database::MoneyManagerDB;
use crate::base_model::BaseModel;
use crate::base_controller::BaseController;
use crate::account::model::{Account, AccountForm, AccountUser};
use crate::user::model::User;

pub mod model;

mod account_type;

#[post("/", data = "<json>", format = "application/json")]
fn create(conn: MoneyManagerDB, json: Json<AccountForm>, user: User) -> Result<Json<Account>, Status> {
    debug!("CREATE_ACCOUNT_REQUEST");
    conn.transaction::<Json<Account>, Error, _>(|| {
        let account = Account::create(&json.into_inner(), &conn)
            .map_err(|e| { error!("Can not create account: {}", e); e})?;
        let au = AccountUser {
            id_user: user.id,
            id_account: account.id,
        };
        AccountUser::create(&au, &conn)
            .map_err(|e| { error!("Can not create account_user: {}", e); e})?;
        info!("account create successfully: {}", account.id);
        Ok(Json(account))
    }).map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
fn read_one(conn: MoneyManagerDB, id: i64, user: User) -> Result<Json<Account>, Status> {
    debug!("READ_ONE_ACCOUNT_REQUEST");
    let account = get_by_id(id, &conn)?;
    // a user can access his own account
    check_property(&conn, &account, &user)?;
    Ok(Json(account))
}

#[get("/user")]
pub fn read_by_user(conn: MoneyManagerDB, user: User) -> Result<Json<Vec<Account>>, Custom<String>> {
    debug!("READ_BY_USER_ACCOUNT_REQUEST");
    let result = Account::read_by_user(&user, &conn);
    Account::unpack(result)
}

#[put("/<id>", data = "<json>", format = "application/json")]
fn update(conn: MoneyManagerDB, id: i64, json: Json<AccountForm>, user: User) -> Result<Status, Status> {
    debug!("UPDATE_ACCOUNT_REQUEST");
    let account = get_by_id(id, &conn)?;
    // check if account can be updated
    check_property(&conn, &account, &user)?;
    let result = Account::update(&account, &json.into_inner(), &conn);
    Account::finalize_update_delete(result)
}

#[delete("/<id>")]
fn delete(conn: MoneyManagerDB, id: i64, user: User) -> Result<Status, Status> {
    debug!("DELETE_ACCOUNT_REQUEST");
    let account = get_by_id(id, &conn)?;
    // check if causal can be deleted
    check_property(&conn, &account, &user)?;
    let result = Account::delete(&account, &conn);
    Account::finalize_update_delete(result)
}

///
///
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/account", routes![read_one, read_by_user, create, update, delete])
}

///
///
pub fn mount_account_type(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/account/type", account_type::get_mount())
}

///
///
pub fn get_and_check(id_account: i64, user: &User, conn: &MoneyManagerDB) -> Result<Account, Status> {
    let account = get_by_id(id_account, conn)?;
    check_property(conn, &account, user)?;
    Ok(account)
}

///
///
pub fn check(id_account: i64, user: &User, conn: &MoneyManagerDB) -> Result<(), Status> {
    check_property_by_id(conn, id_account, user)
}

// #################################################################################################

fn get_by_id(id: i64, conn: &MoneyManagerDB) -> Result<Account, Status> {
    Account::read_by_id(id, &conn)
        .map_err(|e| {
            error!("Can not read account: {}", e);
            if e.eq(&Error::NotFound) {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

fn check_property_by_id(conn: &MoneyManagerDB, id_account: i64, user: &User) -> Result<(), Status> {
    AccountUser::read_for_check(conn, user, id_account)
        .map(|_| Ok(()))
        .map_err(|e| {
            if e.eq(&Error::NotFound) {
                warn!("The user attempts to access account that does not belong to it! {}", e);
                Status::Forbidden
            } else {
                error!("{}", e);
                Status::InternalServerError
            }
        })?
}

fn check_property(conn: &MoneyManagerDB, account: &Account, user: &User) -> Result<(), Status> {
    check_property_by_id(conn, account.id, user)
}
