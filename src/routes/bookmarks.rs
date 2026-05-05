use axum::{Json, extract::State, http::StatusCode};
use sqlx::query_as;

use crate::{
    error::AppError,
    models::{bookmark::Bookmark, request::CreateBookmarkRequest},
    state::AppState,
};

const LIST_BOOKMARKS_QUERY: &str = "SELECT id, url, title FROM bookmarks ORDER BY id";
const CREATE_BOOKMARK_QUERY: &str =
    "INSERT INTO bookmarks (url, title) VALUES ($1, $2) RETURNING id, url, title";

pub async fn create_bookmark(
    State(state): State<AppState>,
    Json(payload): Json<CreateBookmarkRequest>,
) -> Result<(StatusCode, Json<Bookmark>), AppError> {
    let url = payload.url.trim();

    if url.is_empty() {
        return Err(AppError::validation("url must not be empty"));
    }

    let title = payload
        .title
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    let bookmark = query_as::<_, Bookmark>(CREATE_BOOKMARK_QUERY)
        .bind(url)
        .bind(title)
        .fetch_one(&state.pool)
        .await
        .map_err(|error| AppError::database_query(CREATE_BOOKMARK_QUERY, error))?;

    Ok((StatusCode::CREATED, Json(bookmark)))
}

pub async fn list_bookmarks(
    State(state): State<AppState>,
) -> Result<Json<Vec<Bookmark>>, AppError> {
    let bookmarks = query_as::<_, Bookmark>(LIST_BOOKMARKS_QUERY)
        .fetch_all(&state.pool)
        .await
        .map_err(|error| AppError::database_query(LIST_BOOKMARKS_QUERY, error))?;

    Ok(Json(bookmarks))
}
