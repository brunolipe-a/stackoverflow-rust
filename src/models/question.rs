use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::schema::questions;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Question {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl NewQuestion {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            created_at: None,
        }
    }
}
