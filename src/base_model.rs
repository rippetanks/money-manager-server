/*
    Copyright (C) 2019  Simone Martelli

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use diesel::result::Error;

use crate::auth::model::Auth;
use crate::user::model::User;
use crate::causal::model::Causal;
use crate::account::model::{Account, AccountType};
use crate::currency::model::Currency;
use crate::transaction::model::{Transaction, TransactionType, TransactionDetail};
use crate::place::model::Place;
use crate::detail::model::Detail;
use crate::giro::model::Giro;

pub trait BaseModel<T> {
    fn unpack(result: Result<Vec<T>, Error>) -> Result<Json<Vec<T>>, Custom<String>> {
        match result {
            Ok(result) => {
                if result.len() != 0 {
                    Ok(Json(result))
                } else {
                    debug!("Unpack no content!");
                    // Ok(Json(Vec::new()))
                    Err(Custom(Status::NoContent, String::new()))
                }
            },
            Err(e) => {
                error!("An error occurred during unpack: {}", e);
                Err(Custom(Status::InternalServerError, e.to_string()))
            }
        }
    }
}

impl BaseModel<Auth> for Auth { }
impl BaseModel<User> for User { }
impl BaseModel<Causal> for Causal { }
impl BaseModel<Account> for Account { }
impl BaseModel<AccountType> for AccountType { }
impl BaseModel<Currency> for Currency { }
impl BaseModel<Transaction> for Transaction { }
impl BaseModel<TransactionType> for TransactionType { }
impl BaseModel<TransactionDetail> for TransactionDetail { }
impl BaseModel<Place> for Place { }
impl BaseModel<Detail> for Detail { }
impl BaseModel<Giro> for Giro { }
