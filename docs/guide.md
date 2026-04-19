# Actix Web + Diesel Async Starter Guide

A **minimal, production-ready backend starter** for building web services with **Actix Web** and **Diesel** async ORM.

## Project Structure

```
api  →  app  →  infra
```

- **api** — HTTP routes, middlewares, request/response handling, validation
- **app** — Domain logic, business rules, DTOs, error handling
- **infra** — Database schema, models, external services (auth, SMS, Redis), settings

## Quick Start

### 1. Prerequisites

- **Rust 1.95+** — Install from [rustup.rs](https://rustup.rs)
- **PostgreSQL 12+** — Database backend
- **Diesel CLI** — `cargo install diesel_cli --no-default-features --features postgres`

### 2. Setup Environment

Copy the example environment file:

```bash
cp .env.example .env
```

Configure your `.env` with your database connection:

```env
DATABASE_URL=postgres://user:password@localhost:5432/mydb
# Other settings...
```

### 3. Initialize Database

Run migrations:

```bash
diesel migration run
```

Seed initial data:

```bash
pip install psycopg2-binary
python scripts/seed.py
```

### 4. Run the Server

```bash
cargo run
```

Server starts at `http://localhost:8000`

API docs available at `http://localhost:8000/swagger-ui/`

## Architecture & Patterns

### Diesel & Database

- **Zero raw SQL** — Never use `diesel::sql_query`. All queries use the Diesel query builder.
- **Diesel plugins** — Uses `diesel_postgis` for geographic queries (ready to extend).
- **Async runtime** — Uses `diesel_async` for non-blocking database operations.

### API Conventions

- **Response format** — All responses use `send_ok_response()` or `send_created_response()` from `crate::response`
- **DTOs use camelCase** — All request/response DTOs use `#[serde(rename_all = "camelCase")]`
- **Input validation** — All request DTOs derive `#[derive(Validate)]` for automatic validation

### Domain Layer (app/src)

Organize by domain:

```
app/src/domains/
  ├── auth/          # Authentication & login logic
  │   ├── mod.rs
  │   ├── dtos.rs    # LoginStart, LoginComplete, AuthenticatedUser, etc.
  │   └── logic.rs   # login_start(), login_complete(), etc.
  ├── users/         # User management
  │   ├── mod.rs
  │   ├── dtos.rs    # CreateUser, EditUser, UserDto, etc.
  │   └── logic.rs   # get_profile(), update_profile(), etc.
  └── mod.rs         # Domain re-exports
```

### DTO Naming Convention

**Request DTOs** (input from client):
- `Create<X>` — Create a new resource
- `Edit<X>` — Update an existing resource

**Response DTOs**:
- `<X>Dto` — Read-only data representation (for GET responses)
- `<X>Success` — Result of successful operations (for POST/PATCH responses)

Example:

```rust
// app/src/domains/users/dtos.rs
#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateUser {
    pub phone: String,
    pub full_name: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct CreateUserSuccess {
    pub id: Uuid,
    pub phone: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct UserDto {
    pub id: Uuid,
    pub phone: String,
    pub full_name: String,
}
```

### API Routes (api/src/routes)

Each domain gets its own route module:

```
api/src/routes/
  ├── auth/          # /api/v1/auth/* endpoints
  ├── users/         # /api/v1/users/* endpoints
  └── mod.rs         # Route configuration
```

Example endpoint:

```rust
// api/src/routes/users/mod.rs
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = CreateUserSuccess),
        (status = 400, description = "Invalid input"),
    ),
    tag = "Users"
)]
#[post("")]
async fn r_create_user(
    pool: web::Data<DbPool>,
    req: Json<CreateUser>,
) -> HttpResponse {
    let result = app::domains::users::create_user(&pool, req.into_inner()).await;
    response::send_created_response(result)
}
```

## Migrations

All migrations are in `migrations/` directory.

Create a new migration:

```bash
diesel migration generate add_users_table
```

This creates:
- `migrations/<TIMESTAMP>_add_users_table/up.sql` — Apply changes
- `migrations/<TIMESTAMP>_add_users_table/down.sql` — Rollback changes

Run migrations:

```bash
diesel migration run
```

Revert last migration:

```bash
diesel migration revert
```

## Development Workflow

### Adding a New Endpoint

1. **Create database schema** (if needed)
   - Create migration: `diesel migration generate`
   - Write SQL in `up.sql` and `down.sql`
   - Run: `diesel migration run`

2. **Create Diesel model** in `infra/src/models/`
   ```rust
   use crate::schema::my_table;
   
   #[derive(Queryable, Selectable)]
   pub struct MyModel {
       pub id: Uuid,
       pub name: String,
   }
   ```

3. **Create app domain** in `app/src/domains/`
   - Define DTOs in `dtos.rs`
   - Implement business logic in `logic.rs`
   - Export from `mod.rs`

4. **Create API route** in `api/src/routes/`
   - Define handler with `#[post]`, `#[get]`, etc.
   - Add `#[utoipa::path(...)]` for API docs
   - Use response helpers

5. **Register route** in `api/src/routes/mod.rs`
   ```rust
   pub fn configure_routes(cfg: &mut ServiceConfig) {
       cfg.service(web::scope("/my-domain").configure(my_domain::configure_routes));
   }
   ```

### Running Tests

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

### Checking Code Quality

```bash
# Lint checks
cargo clippy --all-targets

# Format check
cargo fmt --check

# Format fix
cargo fmt
```

## Services & Utilities

### Authentication (infra/src/services/auth.rs)

Uses **Paseto v4** for secure token-based authentication:

```rust
use infra::services::auth::AuthService;

// Initialize auth service
let auth_service = AuthService::new(&settings.security)?;

// Verify token
let user = auth_service.verify_token::<AuthenticatedUser>(token_str)?;
```

### Redis (infra/src/services/redis.rs)

For caching and session management:

```rust
use infra::services::redis::RedisService;

let redis = RedisService::new(&settings.redis).await?;
redis.ping().await?;
```

### SMS (infra/src/services/frogsms.rs)

SMS provider integration (Frog SMS API):

```rust
use infra::services::frogsms::FrogSmsProvider;

let sms = FrogSmsProvider::new(&settings.frogsms)?;
sms.send_sms(phone, message).await?;
```

## Configuration

Configuration is managed in `infra/src/settings.rs` and loaded from:

1. `.env` file (takes priority)
2. `config/` directory based on environment
   - `config/default.toml` — Shared settings
   - `config/development.toml` — Development-specific
   - `config/production.toml` — Production-specific

Example `.env`:

```env
DATABASE_URL=postgres://user:password@localhost/mydb
REDIS_URL=redis://localhost:6379
FROGSMS_API_KEY=your_api_key
SERVER_HOST=127.0.0.1
SERVER_PORT=8000
```

## OpenAPI & Swagger UI

The API automatically generates OpenAPI documentation.

View at: `http://localhost:8000/swagger-ui/`

Define endpoints with `#[utoipa::path(...)]`:

```rust
#[utoipa::path(
    post,
    path = "/endpoint",
    request_body = MyRequest,
    responses(
        (status = 200, description = "Success", body = MyResponse),
        (status = 400, description = "Bad request"),
    ),
)]
```

## Error Handling

**App layer errors** (`app/src/error.rs`):

```rust
pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    ValidationError(String),
}
```

**API layer errors** (`api/src/error.rs`):

- Converts app errors to HTTP responses
- Logs server errors while hiding details from clients

## Project Commands

| Command | Purpose |
|---------|---------|
| `cargo build` | Build the project |
| `cargo run` | Run the development server |
| `cargo test` | Run all tests |
| `cargo clippy` | Lint checks |
| `cargo fmt` | Format code |
| `diesel migration run` | Apply migrations |
| `diesel migration revert` | Rollback last migration |
| `python scripts/seed.py` | Seed test data |

## Production Deployment

### Environment Variables

Set appropriate config for production:

```bash
export APP_ENV=production
export DATABASE_URL=postgres://prod_user:prod_pass@prod_db:5432/prod_db
export REDIS_URL=redis://prod_redis:6379
```

### Build Release Binary

```bash
cargo build --release
```

Binary is in `target/release/actix-diesel-async-starter`

### Health Check

```bash
curl http://localhost:8000/api/healthcheck
```

Returns:

```json
{
  "database": true,
  "redis": true
}
```

## Resources

### Diesel
- [Diesel Official Docs](https://diesel.rs)
- [Composing Applications](https://diesel.rs/guides/composing-applications/)
- [diesel_async GitHub](https://github.com/weiznich/diesel_async)

### Actix Web
- [Actix Web Documentation](https://actix.rs/)
- [Request Guards](https://actix.rs/docs/handlers/)

### Rust Async
- [Tokio Runtime](https://tokio.rs)
- [Futures Concurrency](https://docs.rs/futures-util/)

## License

MIT
