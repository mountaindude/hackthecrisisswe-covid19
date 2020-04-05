#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate redis;
extern crate dotenv;
extern crate geo;
extern crate reqwest;

mod db_models;
mod view_models;
mod cors_fairing;
mod apihelper;
mod transaction;
mod schema;
mod geoencoding;

use diesel::prelude::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let mut server = rocket::ignite().mount("/", routes![
        index,
        crate::transaction::getTransaction,
        crate::transaction::getTransactionList
    ]);

    if envmnt::get_or("DEBUG", "0") != "0" {
        server
            .attach(crate::cors_fairing::CORS())
            .launch();
    } else {
        server.launch();
    }
}