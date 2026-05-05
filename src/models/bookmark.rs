use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Bookmark {
    pub id: i32,
    pub url: String,
    pub title: Option<String>,
}
