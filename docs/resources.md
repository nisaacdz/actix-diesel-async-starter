# Resources & References

Useful documentation, guides, and source code for the libraries and patterns used in this project.

## Diesel (Query Builder & ORM)

- [Diesel — Homepage](https://diesel.rs)
- [Diesel — Composing Applications](https://diesel.rs/guides/composing-applications/) — module structure, associations, alias joins
- [Diesel — Extending Diesel](https://diesel.rs/guides/extending-diesel/) — custom types, SQL functions
- [Diesel — Configuring CLI](https://diesel.rs/guides/configuring-diesel-cli/)
- [Diesel — 2.0 Release Notes](https://diesel.rs/news/2_0_0_release/) — alias!, improved joins
- [Diesel — 2.1 Release Notes](https://diesel.rs/news/2_1_0_release/)
- [diesel_async — GitHub](https://github.com/weiznich/diesel_async) — async runtime for Diesel
- [diesel_derive_enum — GitHub](https://github.com/adwhit/diesel-derive-enum) — Postgres enum ↔ Rust enum mapping

## PostGIS & Geography

- [postgis-diesel — GitHub](https://github.com/vitaly-m/postgis-diesel) — Diesel integration for PostGIS types
- [postgis-diesel — Operators source](https://github.com/vitaly-m/postgis-diesel/blob/master/src/operators.rs) — available spatial operators
- [PostGIS — KNN distance operator docs](https://postgis.net/docs/geometry_distance_knn.html) — `<->` operator for nearest-neighbor queries

## PostgreSQL Extensions

- [pg_uuidv7](https://github.com/fboulnois/pg_uuidv7) — time-sortable UUID v7 generation

## Actix Web

- [Actix Web — Docs](https://actix.rs/docs/)
- [utoipa — GitHub](https://github.com/juhaku/utoipa) — OpenAPI/Swagger doc generation from Rust types

## Payment & SMS

- [Paystack API — Docs](https://paystack.com/docs/api/)
- [Hubtel API — Docs](https://developers.hubtel.com/)

## Infrastructure

- [Neon — Serverless Postgres](https://neon.tech/docs) — branching for ephemeral CI databases
- [Redis — Commands](https://redis.io/commands/) — seat locking TTL, Pub/Sub
