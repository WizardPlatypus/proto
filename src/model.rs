use sqlx::types::Uuid;

pub struct Email {
    value: String
}

pub struct User {
    id: Uuid,
    nickname: String,
    email: Email,
    phc: String,
}
