
use rocket::fairing::AdHoc;

use crate::database::MoneyManagerDB;
use crate::causal;
use crate::user;
use crate::auth;
use crate::account;
use crate::currency;
use crate::transaction;
use crate::place;
use crate::detail;
use crate::giro;

#[derive(Debug)]
pub struct Extras {
    pub jwt_key: String,
    pub jwt_exp: u64
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn init(host: &String) {
    let mut rocket = rocket::ignite()
        .attach(MoneyManagerDB::fairing())
        .attach(AdHoc::on_attach("Extras Fairing", |rocket| {
            let config = rocket.config();
            let jwt_key = config.get_str("jwt_key").unwrap().to_string();
            let jwt_exp = config.get_int("jwt_exp").unwrap() as u64;
            Ok(rocket.manage(Extras {
                jwt_key,
                jwt_exp
            }))
        }));

    rocket = rocket.mount(host, routes![index]);
    rocket = causal::mount(rocket);
    rocket = user::mount(rocket);
    rocket = auth::mount(rocket);
    rocket = account::mount(rocket);
    rocket = account::mount_account_type(rocket);
    rocket = currency::mount(rocket);
    rocket = transaction::mount(rocket);
    rocket = transaction::mount_transaction_type(rocket);
    rocket = transaction::mount_transaction_detail(rocket);
    rocket = place::mount(rocket);
    rocket = detail::mount(rocket);
    rocket = giro::mount(rocket);

    rocket.launch();
}
