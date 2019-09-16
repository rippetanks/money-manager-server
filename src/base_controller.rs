
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
