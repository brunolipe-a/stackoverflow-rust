use std::env;

use diesel::prelude::*;
use rocket::{http::Status, response::status::Created, serde::json::Json, Route};

use uuid::Uuid;

use crate::{
    database::{establish_connection, schema},
    models::answer::{Answer, NewAnswer},
    requests::answers::StoreAnswerRequest,
};

#[get("/questions/<question_id>/answers")]
pub fn index(question_id: &str) -> Json<Vec<Answer>> {
    use schema::answers::dsl;

    let question_uuid = Uuid::parse_str(question_id).unwrap_or_default();

    let connection = &mut establish_connection();

    let results = dsl::answers
        .filter(dsl::question_id.eq(question_uuid))
        .select(Answer::as_select())
        .load(connection)
        .expect("Error loading answers");

    Json(results)
}

#[get("/answers/<id>")]
pub fn show(id: &str) -> Result<Json<Answer>, Status> {
    use schema::answers::dsl::answers;

    let uuid = Uuid::parse_str(id).unwrap_or_default();

    let connection = &mut establish_connection();

    let answer: Option<Answer> = answers
        .find(uuid)
        .select(Answer::as_select())
        .first(connection)
        .optional()
        .expect("Error loading answer");

    if answer.is_none() {
        return Err(Status::NotFound);
    }

    Ok(Json(answer.unwrap()))
}

#[post(
    "/questions/<question_id>/answers",
    format = "json",
    data = "<request>"
)]
pub fn store(question_id: &str, request: Json<StoreAnswerRequest>) -> Created<Json<Answer>> {
    use schema::answers;

    let question_uuid = Uuid::parse_str(question_id).unwrap_or_default();

    let connection = &mut establish_connection();

    let new_answer = NewAnswer::new(question_uuid, request.content.to_owned());

    let answer = diesel::insert_into(answers::table)
        .values(&new_answer)
        .returning(Answer::as_returning())
        .get_result(connection)
        .expect("Error saving new answer");

    let app_url = env::var("APP_URL").expect("APP_URL must be set");
    let location = format!("{}/answers/{}", app_url, answer.id);

    Created::new(location).body(Json(answer))
}

#[delete("/answers/<id>")]
pub fn destroy(id: &str) -> Status {
    use schema::answers::dsl::answers;

    let uuid = Uuid::parse_str(&id).unwrap_or_default();

    let connection = &mut establish_connection();

    diesel::delete(answers.find(uuid))
        .execute(connection)
        .expect("Error deleting answer");

    Status::NoContent
}

pub fn routes() -> Vec<Route> {
    routes![index, show, store, destroy]
}
