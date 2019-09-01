
use rocket_contrib::databases::diesel;

#[database("db")]
pub struct MoneyManagerDB(diesel::PgConnection);
