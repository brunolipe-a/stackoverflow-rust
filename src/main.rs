#[macro_use]
extern crate rocket;

mod config;
mod controllers;
mod database;
mod models;
mod requests;

use config::cors::CORS;
use controllers::{answers, questions};
use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/questions", questions::routes())
        .mount("/", answers::routes())
        .attach(CORS)
}
