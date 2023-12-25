use std::env;

use diesel::prelude::*;
use rocket::{http::Status, response::status::Created, serde::json::Json, Route};

use uuid::Uuid;

use crate::{
    database::{establish_connection, schema},
    models::question::{NewQuestion, Question},
    requests::questions::StoreQuestionRequest,
};

#[get("/")]
pub fn index() -> Json<Vec<Question>> {
    use schema::questions::dsl::*;

    let connection = &mut establish_connection();

    let results = questions
        .select(Question::as_select())
        .load(connection)
        .expect("Error loading questions");

    Json(results)
}

#[get("/<id>")]
pub fn show(id: &str) -> Result<Json<Question>, Status> {
    use schema::questions::dsl::questions;

    let uuid = Uuid::parse_str(&id).unwrap_or_default();

    let connection = &mut establish_connection();

    let question: Option<Question> = questions
        .find(uuid)
        .select(Question::as_select())
        .first(connection)
        .optional()
        .expect("Error loading question");

    if question.is_none() {
        return Err(Status::NotFound);
    }

    Ok(Json(question.unwrap()))
}

#[post("/", format = "json", data = "<request>")]
pub fn store(request: Json<StoreQuestionRequest>) -> Created<Json<Question>> {
    use schema::questions;

    let connection = &mut establish_connection();

    let new_question = NewQuestion::new(request.title.to_owned(), request.description.to_owned());

    let question = diesel::insert_into(questions::table)
        .values(&new_question)
        .returning(Question::as_returning())
        .get_result(connection)
        .expect("Error saving new questions");

    let app_url = env::var("APP_URL").expect("APP_URL must be set");
    let location = format!("{}/questions/{}", app_url, question.id);

    Created::new(location).body(Json(question))
}

#[delete("/<id>")]
pub fn destroy(id: &str) -> Status {
    use schema::questions::dsl::questions;

    let uuid = Uuid::parse_str(&id).unwrap_or_default();

    let connection = &mut establish_connection();

    diesel::delete(questions.find(uuid))
        .execute(connection)
        .expect("Error deleting questions");

    Status::NoContent
}

pub fn routes() -> Vec<Route> {
    routes![index, show, store, destroy]
}
