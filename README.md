# Actix Web + Diesel Async Starter

A robust, production-ready backend starter template built with **Actix Web**, **Diesel (async)**, and **PostgreSQL**.

## Getting Started

1. Configure `.env` with your `DATABASE_URL`
2. Run `diesel migration run`
3. Run `cargo run`

## Architecture

```
api  →  app  →  infra
```

- **api** — HTTP routes, middlewares, request validation, cookie handling. Framework-specific code stays here.
- **app** — Domain logic, business rules, DTOs. Queries the database directly through `infra` using Diesel's type-safe query builder.
- **infra** — Database schema, external services (storage, Redis), settings, auth token engine.

### Rules

#### Layer discipline
- Push all errors as close to startup or the api layer as possible (custom deserialization in `settings.rs`, `expect` in service creation, middleware validation).
- Convert framework-specific types to intermediate types before passing from api to app.
- Use `infra` only for low-level setup that requires direct external dependencies and does not depend on other infra parts (excluding `Settings`).

#### Error handling
- `AppError` separates server-logged errors from user-facing error messages. Use `.message()` for user-friendly overrides; use `.detail()` for structured error data.
- Use `AppError::internal(err).message("user friendly")` pattern for external service failures.

#### Diesel & database
- **Zero raw SQL** — never use `diesel::sql_query`. All queries must go through the Diesel query builder, including complex multi-alias joins. This is a hard rule.
- Use `diesel::alias!` for self-joins on the same table.
- Each query should be efficient — avoid unnecessary DB round trips.
- All paginated list endpoints must fetch count and data **concurrently** using `futures_util::try_join!` with two separate connections from the pool.
- Use `diesel_derive_enum::DbEnum` for Postgres enums; map them 1:1 to Rust enums.

#### API conventions
- In `api`, for `Path` params, avoid using `{id}`. Instead, use specific IDs like `{user_id}` to avoid confusion.
- All DTOs use `#[serde(rename_all = "camelCase")]` — the API speaks camelCase.
- Query params for list endpoints use `utoipa::IntoParams` for OpenAPI docs.

#### General
- No unmaintained / deprecated crates.
- Always use latest recommended APIs — cross-check docs.

## Auth

Stateless encrypted cookie sessions using `XChaCha20Poly1305` and `bitcode` serialization with a sliding token strategy:

- **`iat`** — Issued-at timestamp. If age exceeds `refresh_threshold` → refresh token (DB lookup rebuilds payload with fresh data).
- **`exp`** — Expiry timestamp. If past → reject (must re-authenticate).
- On refresh: both `iat` and `exp` reset, extending the session.