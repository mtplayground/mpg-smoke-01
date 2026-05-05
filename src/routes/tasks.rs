use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::error::AppError;
use crate::models::request::{CreateTaskRequest, UpdateTaskRequest};
use crate::models::task::Task;
use crate::state::AppState;

const LIST_TASKS_QUERY: &str = "SELECT id, title, completed FROM tasks ORDER BY id";
const CREATE_TASK_QUERY: &str =
    "INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, completed";
const UPDATE_TASK_QUERY: &str =
    "UPDATE tasks SET completed = $2 WHERE id = $1 RETURNING id, title, completed";
const DELETE_TASK_QUERY: &str = "DELETE FROM tasks WHERE id = $1";

pub async fn list_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = sqlx::query_as::<_, Task>(LIST_TASKS_QUERY)
        .fetch_all(&state.pool)
        .await
        .map_err(|error| AppError::database_query(LIST_TASKS_QUERY, error))?;

    Ok(Json(tasks))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(AppError::validation("title must not be empty"));
    }

    let task = sqlx::query_as::<_, Task>(CREATE_TASK_QUERY)
        .bind(title)
        .fetch_one(&state.pool)
        .await
        .map_err(|error| AppError::database_query(CREATE_TASK_QUERY, error))?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update_task(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, AppError> {
    let task = sqlx::query_as::<_, Task>(UPDATE_TASK_QUERY)
        .bind(id)
        .bind(payload.completed)
        .fetch_optional(&state.pool)
        .await
        .map_err(|error| AppError::database_query(UPDATE_TASK_QUERY, error))?
        .ok_or_else(|| AppError::not_found(format!("task {id} not found")))?;

    Ok(Json(task))
}

pub async fn delete_task(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query(DELETE_TASK_QUERY)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|error| AppError::database_query(DELETE_TASK_QUERY, error))?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found(format!("task {id} not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}
