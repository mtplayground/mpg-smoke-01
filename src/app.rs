use axum::{
    Router,
    routing::{get, patch},
};

use crate::{
    routes::{
        bookmarks::{create_bookmark, list_bookmarks},
        tasks::{create_task, delete_task, list_tasks, update_task},
    },
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", patch(update_task).delete(delete_task))
        .route("/bookmarks", get(list_bookmarks).post(create_bookmark))
        .with_state(state)
}
