# Product

`mpg-smoke-01` is a minimal Rust `axum` service backed by PostgreSQL.

## What It Does

- Serves a JSON HTTP API on `0.0.0.0:8080`.
- Persists all application data in PostgreSQL via `sqlx`.
- Runs embedded SQL migrations at startup before accepting traffic.

## Current API

- `GET /tasks`
  Returns all tasks ordered by `id`.
- `POST /tasks`
  Accepts `{ "title": string }` and returns the created task.
- `PATCH /tasks/:id`
  Accepts `{ "completed": boolean }` and returns the updated task.
- `DELETE /tasks/:id`
  Deletes the task and returns `204 No Content`.
- `GET /bookmarks`
  Returns all bookmarks ordered by `id`.
- `POST /bookmarks`
  Accepts `{ "url": string, "title": string | null }` and returns the created bookmark.

Task shape:
- `id: integer`
- `title: string`
- `completed: boolean`

Bookmark shape:
- `id: integer`
- `url: string`
- `title: string | null`

## Architecture

- Runtime: Rust + `axum` + `tokio`
- Database: PostgreSQL only
- Data access: direct `sqlx` queries over one shared `PgPool`
- Schema: `tasks` and `bookmarks` tables, each created by SQL migrations in `migrations/`
- Startup flow: load env, configure a PgBouncer-safe Postgres connection, run embedded migrations, then serve traffic
- Errors: centralized HTTP error mapping with database failures logged through `tracing`

## Conventions

- `DATABASE_URL` is required.
- `PORT` is optional and defaults to `8080`.
- Empty task titles and empty bookmark URLs are rejected.
- The codebase is a single backend service, not a monorepo.

## Deployment Shape

- Includes a multi-stage Docker build.
- Builder image: `rust:1.82`
- Runtime image: `debian:bookworm-slim`
