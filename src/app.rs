use axum::{Router, routing::{get, patch}};

use crate::routes::tasks::{create_task, delete_task, list_tasks, update_task};
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", patch(update_task).delete(delete_task))
        .with_state(state)
}
