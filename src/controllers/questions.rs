use chrono::{SecondsFormat, Utc};
use rocket::{
    http::Status,
    response::status::Created,
    serde::{json::Json, Serialize},
    Route,
};

use uuid::Uuid;

use crate::requests::questions::StoreQuestionRequest;

#[derive(Serialize)]
pub struct QuestionResource {
    id: String,
    title: String,
    description: String,
    created_at: String,
}

#[get("/")]
pub fn index() -> Json<Vec<QuestionResource>> {
    Json(vec![QuestionResource {
        id: Uuid::new_v4().to_string(),
        description: "Test 1".to_owned(),
        title: "test 2".to_owned(),
        created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    }])
}

#[post("/", format = "json", data = "<request>")]
pub fn store(request: Json<StoreQuestionRequest>) -> Created<Json<QuestionResource>> {
    let json = Json(QuestionResource {
        id: Uuid::new_v4().to_string(),
        description: request.description.to_owned(),
        title: request.title.to_owned(),
        created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    });

    Created::new("").body(json)
}

#[delete("/<id>")]
pub fn destroy(id: String) -> Status {
    Status::NoContent
}

#[get("/<id>")]
pub fn show(id: String) -> Json<QuestionResource> {
    Json(QuestionResource {
        id,
        description: "tera".to_owned(),
        title: "terasd".to_owned(),
        created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    })
}

pub fn routes() -> Vec<Route> {
    routes![index, show, store, destroy]
}
