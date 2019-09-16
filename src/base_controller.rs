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

use rocket::http::Status;
use diesel::QueryResult;

use crate::auth::model::Auth;
use crate::user::model::User;
use crate::causal::model::Causal;
use crate::account::model::{Account, AccountType};
use crate::currency::model::Currency;
use crate::transaction::model::{Transaction, TransactionType, TransactionDetail};
use crate::place::model::Place;
use crate::detail::model::Detail;
use crate::giro::model::Giro;

pub trait BaseController<T> {
    fn finalize_update_delete(result: QueryResult<usize>) -> Result<Status, Status> {
        match result {
            Ok(n) if n > 0 => Ok(Status::NoContent),
            Ok(_) => {
                warn!("object not found!");
                Err(Status::NotFound)
            },
            Err(e) => {
                error!("error on update/delete object: {}", e);
                Err(Status::InternalServerError)
            }
        }
    }
}

impl BaseController<Auth> for Auth { }
impl BaseController<User> for User { }
impl BaseController<Causal> for Causal { }
impl BaseController<Account> for Account { }
impl BaseController<AccountType> for AccountType { }
impl BaseController<Currency> for Currency { }
impl BaseController<Transaction> for Transaction { }
impl BaseController<TransactionType> for TransactionType { }
impl BaseController<TransactionDetail> for TransactionDetail { }
impl BaseController<Place> for Place { }
impl BaseController<Detail> for Detail { }
impl BaseController<Giro> for Giro { }
