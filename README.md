# URL Shortener

A Rust URL shortener API built with Axum and MySQL. Create short links, redirect by code, track clicks, and authenticate users with JWT.

## Features

- Shorten URLs into unique short codes
- Redirect from `/{code}` to the original URL
- Async click-count updates on redirect
- User registration and login (Argon2 password hashing)
- JWT-protected shorten endpoint
- SQL migrations via SQLx on startup

## Stack

| Layer | Technology |
| --- | --- |
| HTTP | [Axum](https://github.com/tokio-rs/axum) |
| Runtime | Tokio |
| Database | MySQL + [SQLx](https://github.com/launchbadge/sqlx) |
| Auth | JWT (`jsonwebtoken`) + Argon2 |
| Logging | `tracing` / `tracing-subscriber` |

## Prerequisites

- Rust (edition 2024 toolchain)
- MySQL 8+
- A database created for the app (e.g. `url_shortner`)

## Setup

1. Clone the repo and enter the project directory.

2. Create a `.env` file in the project root:

```env
DATABASE_URL=mysql://root:password@localhost:3306/url_shortner
HOST_URL=127.0.0.1:3000
RUST_LOG=debug
JWT_SECRET=change-me-to-a-long-random-secret
```

3. Ensure MySQL is running and the database exists:

```sql
CREATE DATABASE url_shortner;
```

4. Build and run:

```bash
cargo run
```

Migrations under `migrations/` are applied automatically on startup.

## API

Base URL defaults to `http://127.0.0.1:3000`.

### Health

```http
GET /
```

Returns `200 OK`.

### Auth

#### Register user

```http
POST /auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secret"
}
```

#### Register admin

```http
POST /auth/register-admin
Content-Type: application/json

{
  "email": "admin@example.com",
  "password": "secret"
}
```

#### Login

```http
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secret"
}
```

Returns a JWT string (JSON).

### Shorten (authenticated)

```http
POST /shorten
Authorization: <jwt>
Content-Type: application/json

{
  "url": "https://example.com/very/long/path"
}
```

Response:

```json
{
  "url": "<short_code>"
}
```

### Redirect

```http
GET /{code}
```

Responds with a temporary redirect (`307`) to the original URL and increments `click_count` asynchronously.

## Example flow

```bash
# Register
curl -s -X POST http://127.0.0.1:3000/auth/register \
  -H 'Content-Type: application/json' \
  -d '{"email":"user@example.com","password":"secret"}'

# Login (capture JWT)
TOKEN=$(curl -s -X POST http://127.0.0.1:3000/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"user@example.com","password":"secret"}' | tr -d '"')

# Shorten
curl -s -X POST http://127.0.0.1:3000/shorten \
  -H "Authorization: $TOKEN" \
  -H 'Content-Type: application/json' \
  -d '{"url":"https://example.com"}'

# Redirect (use curl -v or -L to follow)
curl -v http://127.0.0.1:3000/<short_code>
```

## Project layout

```
src/
  handlers/     # HTTP handlers
  services/     # Business logic
  repository/   # Database queries
  models/       # Domain models
  dto/          # Request/response types + app state
  extractors/   # Custom Axum extractors (JWT auth)
  router/       # Route wiring
  migrations/   # SQLx migrations (project root)
```

## Notes

- Short codes are derived from the original URL (MD5-based).
- Use a temporary redirect for click tracking; permanent redirects are often cached by browsers and skip your server on repeat visits.
- Keep `JWT_SECRET` private and use a strong value in production.
