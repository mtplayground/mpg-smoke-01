use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookmarkRequest {
    pub url: String,
    pub title: Option<String>,
}
