use chrono::{SecondsFormat, Utc};
use rocket::{
    http::Status,
    response::status::Created,
    serde::{json::Json, Serialize},
    Route,
};

use uuid::Uuid;

use crate::requests::answers::StoreAnswerRequest;

#[derive(Serialize)]
pub struct AnswerResource {
    id: String,
    question_id: String,
    content: String,
    created_at: String,
}

#[get("/")]
pub fn index() -> Json<Vec<AnswerResource>> {
    Json(vec![AnswerResource {
        id: Uuid::new_v4().to_string(),
        question_id: "Test 1".to_owned(),
        content: "test 2".to_owned(),
        created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    }])
}

#[post("/", format = "json", data = "<request>")]
pub fn store(request: Json<StoreAnswerRequest>) -> Created<Json<AnswerResource>> {
    let json = Json(AnswerResource {
        id: Uuid::new_v4().to_string(),
        question_id: request.question_id.to_owned(),
        content: request.content.to_owned(),
        created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    });

    Created::new("").body(json)
}

#[delete("/<id>")]
pub fn destroy(id: String) -> Status {
    Status::NoContent
}

#[get("/<id>")]
pub fn show(id: String) -> Json<AnswerResource> {
    Json(AnswerResource {
        id,
        question_id: "tera".to_owned(),
        content: "terasd".to_owned(),
        created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    })
}

pub fn routes() -> Vec<Route> {
    routes![index, show, store, destroy]
}
