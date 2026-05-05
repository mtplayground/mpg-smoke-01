# Product

`mpg-smoke-01` is a minimal Rust web service for managing tasks.

## What It Does

- Exposes a JSON CRUD API over HTTP on `0.0.0.0:8080`.
- Persists all task data in PostgreSQL via `sqlx`.
- Creates and maintains the `tasks` table through embedded startup migrations.

## Current API

- `GET /tasks`
  Returns all tasks ordered by `id`.
- `POST /tasks`
  Accepts `{ "title": string }` and returns the created task.
- `PATCH /tasks/:id`
  Accepts `{ "completed": boolean }` and returns the updated task.
- `DELETE /tasks/:id`
  Deletes the task and returns `204 No Content`.

Task shape:
- `id: integer`
- `title: string`
- `completed: boolean`

## Architecture

- Runtime: Rust + `axum` + `tokio`
- Database: PostgreSQL only
- Data access: direct `sqlx` queries with a shared `PgPool`
- Startup flow: read `DATABASE_URL`, connect to Postgres, run embedded migrations, then serve HTTP
- Errors: centralized HTTP error mapping with database failures logged through `tracing`

## Conventions

- The service expects `DATABASE_URL` in the environment.
- `PORT` is optional and defaults to `8080`.
- Input validation is intentionally minimal; empty task titles are rejected.
- This repository currently represents a single-service backend, not a monorepo or full-stack app.

## Deployment Shape

- Includes a multi-stage Docker build.
- Builder image: `rust:1.82`
- Runtime image: `debian:bookworm-slim`
