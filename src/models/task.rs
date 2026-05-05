use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
