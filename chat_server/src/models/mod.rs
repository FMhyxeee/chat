mod chat;
mod file;
mod message;
mod user;
mod workspace;

pub use chat::{CreateChat, UpdateChat};
pub use message::{CreateMessage, ListMessages};
pub use user::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFile {
    pub ws_id: u64,
    pub ext: String, // extract ext from filename or mime type
    pub hash: String,
}
