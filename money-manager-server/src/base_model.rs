
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::auth::model::Auth;
use crate::user::model::User;
use crate::causal::model::Causal;
use crate::account::model::{Account, AccountType};
use crate::currency::model::Currency;

pub trait BaseModel<T> {
    fn unpack(result: Result<Vec<T>, Error>) -> Result<Json<Vec<T>>, Custom<String>> {
        match result {
            Ok(result) => {
                if result.len() != 0 {
                    Ok(Json(result))
                } else {
                    Err(Custom(Status::NoContent, String::new()))
                }
            },
            Err(e) => Err(Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

impl BaseModel<Auth> for Auth { }
impl BaseModel<User> for User { }
impl BaseModel<Causal> for Causal { }
impl BaseModel<Account> for Account { }
impl BaseModel<AccountType> for AccountType { }
impl BaseModel<Currency> for Currency { }
