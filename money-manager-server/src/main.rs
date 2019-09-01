
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

#[macro_use] extern crate log;
extern crate log4rs;

mod controller;
mod base_model;
mod database;
mod schema;

mod auth;
mod causal;
mod user;

fn main() {
    log4rs::init_file("log-config.yml", Default::default()).unwrap();

    let host: String = "/".to_owned();
    controller::init(&host);
}
