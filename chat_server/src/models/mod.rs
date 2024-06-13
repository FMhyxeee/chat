mod user;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub fullname: String,
    pub email: String,
    #[serde(skip)]
    #[sqlx(default)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Chats {
    pub id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Messages {
    pub id: usize,
    pub chat_id: usize,
    pub sender_id: usize,
    pub content: String,
    pub images: Vec<String>,
    pub created_at: DateTime<Utc>,
}
