# Actix Web + Diesel Async Starter Template

This is a starter template for building high-performance web applications using Rust, Actix Web, and Diesel Async.

## Features

- **Actix Web**: Powerful, pragmatic, and extremely fast web framework for Rust.
- **Diesel Async**: Asynchronous database interaction using Diesel.
- **Workspace Structure**: Organized into `api`, `domain`, and `infrastructure` crates for better separation of concerns.
- **Docker Support**: Ready-to-use Dockerfile for containerization.

## Getting Started

### Prerequisites

- Rust (latest stable)
- Docker (optional, for running Postgres)
- PostgreSQL

### Setup

1. Clone the repository.
2. Copy `.env.example` to `.env` and update the values.
   ```bash
   cp .env.example .env
   ```
3. Start the database (if using Docker):
   ```bash
   docker run --name postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=db_name -p 5432:5432 -d postgres
   ```
4. Install Diesel CLI (with postgres features):
   ```bash
   cargo install diesel_cli --no-default-features --features postgres
   ```
5. Run migrations:
   ```bash
   diesel setup
   diesel migration run
   ```
6. Run the server:
   ```bash
   cargo run
   ```

## Project Structure

- `api`: Contains the application entry point, routes, and HTTP handlers.
- `domain`: Contains the business logic and data models.
- `infrastructure`: Contains database connections and external service integrations.
- `migrations`: Database migrations.

## Documentation

See the `docs` folder for more detailed information.
