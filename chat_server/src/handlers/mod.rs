mod auth;
mod chat;
mod messages;
mod workspace;

pub(crate) use auth::*;
pub(crate) use chat::*;
pub(crate) use messages::*;
pub(crate) use workspace::*;

pub(crate) async fn index_handler() -> &'static str {
    "index"
}
