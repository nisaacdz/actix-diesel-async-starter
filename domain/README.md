# Project Documentation

## Folder Structure

This project follows a Clean Architecture-inspired structure, separating concerns into distinct crates:

### `api`
The `api` crate is the entry point of the application. It handles:
- HTTP server setup (Actix Web).
- Routing and request handling.
- Dependency injection (e.g., database pool).
- DTOs (Data Transfer Objects) for API requests/responses.

### `domain`
The `domain` crate contains the core business logic and data models. It should be independent of external frameworks and libraries as much as possible.
- Entities/Models.
- Business rules.
- Interfaces for repositories (if using the repository pattern).

### `infrastructure`
The `infrastructure` crate implements the interfaces defined in `domain` and handles external concerns.
- Database connection (Diesel Async).
- External API clients.
- File system access.

### `migrations`
Contains SQL migration files managed by Diesel CLI.

## Workflow

1. **Define Domain Models**: Start by defining your data structures in `domain`.
2. **Create Migrations**: Use `diesel migration generate <name>` to create new migrations.
3. **Implement Infrastructure**: Implement database queries in `infrastructure` or directly in `api` handlers if the logic is simple.
4. **Expose API**: Create routes and handlers in `api` to expose the functionality.
