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

use rocket::fairing::AdHoc;
use rocket::error::LaunchError;

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

/*
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
*/

pub fn init() -> LaunchError {
    let mut rocket = rocket::ignite()
        .attach(MoneyManagerDB::fairing())
        .attach(fairing_extra());

    // rocket = rocket.mount(host, routes![index]);
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

    rocket.launch()
}

fn fairing_extra() -> rocket::fairing::AdHoc {
    AdHoc::on_attach("Extras Fairing", |rocket| {
        let config = rocket.config();
        let jwt_key = config.get_str("jwt_key").unwrap().to_string();
        let jwt_exp = config.get_int("jwt_exp").unwrap() as u64;
        Ok(rocket.manage(Extras {
            jwt_key,
            jwt_exp
        }))
    })
}
