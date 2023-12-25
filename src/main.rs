#[macro_use]
extern crate rocket;

mod config;
mod controllers;
mod requests;

use config::cors::CORS;
use controllers::{answers, questions};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/questions", questions::routes())
        .mount("/answers", answers::routes())
        .attach(CORS)
}
