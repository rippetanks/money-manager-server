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

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

#[macro_use] extern crate log;
extern crate log4rs;

use std::path::Path;

mod controller;
mod base_model;
mod base_controller;
mod database;
mod schema;

mod auth;
mod causal;
mod user;
mod account;
mod currency;
mod transaction;
mod place;
mod detail;
mod giro;

fn main() {
    let path = if cfg!(windows) {
        "log-config.yml"
    } else {
        let path = "/etc/money-manager/log-config.yml";
        if Path::new(path).exists() {
            path
        } else {
            "log-config.yml"
        }
    };
    log4rs::init_file(path, Default::default()).unwrap();

    let error = controller::init(); // return only on error
    error!("Launch failed! Error: {}", error);
}
