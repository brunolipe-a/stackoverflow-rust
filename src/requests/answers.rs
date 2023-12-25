use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct StoreAnswerRequest<'r> {
    pub question_id: &'r str,
    pub content: &'r str,
}
