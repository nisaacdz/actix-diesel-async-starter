# Actix Diesel Async Starter

Welcome.

## The Story

I didn't set out to build a starter template. I just wanted to build an application.

My requirements were specific: **Actix Web**, **Diesel**, and **Diesel Async**. I know, I know. Most people just use Diesel with `r2d2` and call it a day. But I couldn't stand the blocking nature of it. It felt wrong to have a highly concurrent web server waiting on synchronous database threads. Other projects just use `sqlx`, but I don't want to use it either. I wanted true async, and I wanted to use `diesel`.

So I went looking. I searched everywhere for a solid template that brought these pieces together. I found a few, but they were either outdated, broken, or just didn't feel right.

Then came the real headache: **TLS**.

I tried the naive approach. I used `AsyncPgConnection::establish`. It looked innocent enough. But on my Windows machine, it greeted me with this:

```
thread 'main' (18708) panicked at migrator\src\main.rs:23:10:
Failed to connect to database: CouldntSetupConfiguration(DatabaseError(UnableToSendCommand, "error performing TLS handshake"))
```

I spent hours debugging this. It turns out, getting `diesel_async` to play nice with TLS isn't as straightforward as the docs make it seem.

So, I had to roll up my sleeves. I ended up writing a custom connection implementation using `rustls`, `tokio-postgres`, and `webpki-roots` using the `ManagerConfig.custom_setup` (Who does that?), just to get past the TLS issues. It was a lot of code for something that should be simple — Well, I shouldn't have expected to find it simple; after all, it's not JavaScript.(Big kudos to Gemini 3 for the assist on the implementation details). The result? A fully async, stable TLS connection that actually works.

## Why This Exists

This repository is the result of that frustration. It's the starter I wish I had found.

I also took the liberty of spicing up the architecture. I was tired of seeing the same old patterns everywhere, so I designed this around a "Double Generic `ApiResponse`" structure, where each endpoint designs its own response success and failure structures.

### The Architecture

-   **`app/`**: The domain logic and types.
-   **`infra/`**: The database and external services.
-   **`api/`**: The HTTP layer.

The core idea is simple:
1.  **`infra`** returns a `Result<S, E>`.
2.  **`api`** converts that into an `ApiResponse<S, E>`.
3.  **`ApiResponse`** handles the conversion to `HttpResponse`.

It keeps the concerns beautifully separated.

## Documentation

1. **`utoipa`**: For open-api documentaion
2. **`docs`**: Jot anything that needs to be jot down.

## Getting Started

1.  Clone the repo.
2.  Check `.env.example` and set up your `.env`.
3.  Run `cargo run`.

I hope this saves you the headache I went through. Enjoy.
