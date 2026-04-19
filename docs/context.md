## Architecture

```
api  →  app  →  infra
```

- **api** — HTTP routes, middlewares, request validation, cookie handling.
- **app** — Domain logic, business rules, DTOs.
- **infra** — Database schema, external services (SMS, Redis), settings, auth.

## Rules

#### Diesel & Database
- **Zero raw SQL** — never use `diesel::sql_query`. All queries must use the Diesel query builder, including complex multi-alias joins. This is a hard rule.
- Use Diesel plugins as needed (e.g., `diesel_postgis` for geographic queries, `diesel_derive_enum` for enum types).
- Use `diesel::alias!` for self-joins on the same table.
- All paginated list endpoints must fetch count and data **concurrently** using `futures_util::try_join!` with two separate connections.

#### API Conventions
- Use `send_ok_response()` and `send_created_response()` defined in `crate::response` for all responses — no `json!()` macro.
- Path params should be explicit: use `{user_id}` or `{operator_id}` instead of generic `{id}`.
- All DTOs use `#[serde(rename_all = "camelCase")]`.
- All request DTOs derive `#[derive(Validate)]` for input validation.

#### Error Handling
- Push all errors as close to startup or the API layer as possible.
- `AppError` context separates server-logged errors from user-facing messages.

## DTO & API Conventions

### Naming Convention

**App Layer DTOs** (in `app/src/domains/*/dtos.rs`):

- **`Create<X>`** — Request struct for creating new `X`
  - Example: `CreateRoute`, `CreateVehicle`
  - Only contains fields required for creation
  
- **`Edit<X>`** — Request struct for updating existing `X`
  - Example: `EditVehicle`, `EditRoutePricing`
  - Uses `Option<Option<T>>` for nullable field updates (with `deserialize_some` helper from `serde_utils`)
  
- **`List<X>`** — Query parameters struct for paginated list endpoints
  - Example: `ListRoutes`, `ListOperatorVehicles`
  - Contains `limit`, `offset`, optional `search`, etc.
  - Clamps values and provides `.limit()` / `.offset()` accessor methods
  
- **`<X>Success`** — Response struct for successful app layer operations
  - Example: `CreateRouteSuccess`, `EditVehicleSuccess`, `DeleteRouteSuccess`
  
- **`<X>Dto`** — Read-only data representation of `X`
  - Example: `RouteDto`, `VehicleDto`
  - Used for GET responses (both list and detail endpoints)

**API Layer Types** (in `api/src/routes/*/mod.rs`):

Only create if you need custom validation or transformation logic:

- **`<X>Request`** — API-specific request wrapper
  - Use ONLY if API needs validation/conversion before passing to app
  - Otherwise use app `Create<X>` type directly via `Json` extractor
  
- **`<X>Response`** — API-specific response wrapper
  - Use ONLY if API needs to transform app result into different type
  - Otherwise return app result (`CreateXSuccess`, etc.) directly

### HTTP Method Patterns

| Method | Request DTO | Response DTO | Example |
|--------|-------------|--------------|---------|
| `POST` | `Create<X>` | `Create<X>Success` | POST /routes → `CreateRoute` → `CreateRouteSuccess` |
| `PATCH` | `Edit<X>` | `Edit<X>Success` | PATCH /routes/{id} → `EditRoute` → `EditRouteSuccess` |
| `GET` (list) | `List<X>` | `Paginated<XDto>` | GET /routes?limit=10 → via `List<X>` params → `Paginated<RouteDto>` |
| `GET` (detail) | — | `<X>Dto` | GET /routes/{id} → (no DTO) → `RouteDto` |
| `DELETE` | — | `Delete<X>Success` | DELETE /routes/{id} → (no DTO) → `DeleteRouteSuccess { id, deleted_at }` |

**Key Rules:**
- **No `PUT` endpoints** — use `PATCH` for all updates
- **All request DTOs** derive `#[derive(Validate)]`
- **All DTOs** use `#[serde(rename_all = "camelCase")]`
- **All responses** use `send_ok_response()` or `send_created_response()` — never `json!()` macro
- **Paginated responses** use `Paginated<T>` with count and data fetched concurrently


## Resources

Useful documentation, guides, and source code for the libraries and patterns used in this project.

### Diesel (Query Builder & ORM)

- [Diesel — Homepage](https://diesel.rs)
- [Diesel — Composing Applications](https://diesel.rs/guides/composing-applications/) — module structure, associations, alias joins
- [Diesel — Extending Diesel](https://diesel.rs/guides/extending-diesel/) — custom types, SQL functions
- [Diesel — Configuring CLI](https://diesel.rs/guides/configuring-diesel-cli/)
- [Diesel — 2.0 Release Notes](https://diesel.rs/news/2_0_0_release/) — alias!, improved joins
- [Diesel — 2.1 Release Notes](https://diesel.rs/news/2_1_0_release/)
- [diesel_async — GitHub](https://github.com/weiznich/diesel_async) — async runtime for Diesel
- [diesel_derive_enum — GitHub](https://github.com/adwhit/diesel-derive-enum) — Postgres enum ↔ Rust enum mapping

### PostGIS & Geography

- [postgis-diesel — GitHub](https://github.com/vitaly-m/postgis-diesel) — Diesel integration for PostGIS types
- [postgis-diesel — Operators source](https://github.com/vitaly-m/postgis-diesel/blob/master/src/operators.rs) — available spatial operators
- [PostGIS — KNN distance operator docs](https://postgis.net/docs/geometry_distance_knn.html) — `<->` operator for nearest-neighbor queries

### PostgreSQL Extensions

- [pg_uuidv7](https://github.com/fboulnois/pg_uuidv7) — time-sortable UUID v7 generation

### Actix Web

- [Actix Web — Docs](https://actix.rs/docs/)
- [utoipa — GitHub](https://github.com/juhaku/utoipa) — OpenAPI/Swagger doc generation from Rust types

### Infrastructure

- [Neon — Serverless Postgres](https://neon.tech/docs) — branching for ephemeral CI databases
- [Redis — Commands](https://redis.io/commands/) — seat locking TTL, Pub/Sub
