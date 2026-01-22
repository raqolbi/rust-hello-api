# Hello API (Rust + Axum)

Hello API is a **minimal, production-ready JSON API** built with **Rust** and **Axum**, designed as a clean starting point for backend services.

The project follows modern backend best practices:

- Environment-driven configuration
- Docker-first deployment
- Hardened runtime defaults
- Clear separation between generator (DevOps) and application (Rust)

---

## ✨ Features

- ⚡ **Axum-based HTTP API** (fast & async)
- 📦 **JSON-only responses** (consistent API contract)
- 🌱 **Environment-based configuration** (`.env`)
- 🩺 **Healthcheck endpoint** (`/health`)
- 🔄 **Graceful shutdown** (SIGTERM / Ctrl+C)
- 🐳 **Docker-ready & hardened** (non-root, optional read-only FS)
- 🧱 **Generator-driven Docker setup** (no manual Docker edits)

---

## 📂 Project Structure

```
hello-api/
├── src/
│   └── main.rs            # Axum application entrypoint
├── Cargo.toml
├── Cargo.lock
├── .env.example           # Example environment configuration
└── .gitignore
```

---

## 🚀 API Endpoints

| Method | Path      | Description            |
| ------ | --------- | ---------------------- |
| GET    | `/`       | Hello World (JSON)     |
| GET    | `/api`    | Hello API (JSON)       |
| GET    | `/health` | Healthcheck (HTTP 200) |

### Example Response

```json
{
  "status": "success",
  "message": "Hello API",
  "data": {}
}
```

---

## ⚙️ Environment Variables

All configuration is provided via environment variables.

### Required

```env
DATABASE_URL=sqlite:///data/db.sqlite
```

### Optional

```env
APP_PORT=8080
LOG_LEVEL=info
GRACEFUL_SHUTDOWN_TIMEOUT=10
```

> The application **never reads config files directly** — only final environment variables.

---

## 🐳 Running with Docker (Recommended)

### 1. Prepare environment

Copy and adjust the environment file:

```bash
cp .env.example .env
```

### 2. Generate Docker artifacts

Docker files are generated via the external generator project (https://github.com/raqolbi/rust-api-docker-generator):

```bash
./setup.sh
```

This will:

- Sync `.env` into the Rust project
- Generate `Dockerfile`
- Generate `docker-compose.yml`

### 3. Build & run

```bash
docker compose up --build
```

API will be available at:

```
http://localhost:8080
```

---

## 🧪 Running Locally (Without Docker)

```bash
export $(cat .env | xargs)
cargo run
```

---

## 🔒 Security Notes

- Runs as **non-root user** inside container
- Supports **read-only root filesystem**
- Secrets are never baked into the image
- Healthcheck is HTTP-based and fast

---

## 📦 Tech Stack

- Rust (Edition 2024)
- Axum
- Tokio
- Serde
- Tracing
- Docker / Docker Compose

---

## 🎯 Design Philosophy

- **Fail fast** on misconfiguration
- **One source of truth** for runtime config
- **No magic**: everything explicit
- **Production-first**, dev-friendly

---

## 📜 License

MIT License

---

## 👋 Next Steps

This project is intended as a foundation. Common extensions include:

- Database integration (SQLx)
- Authentication (JWT)
- Readiness checks (`/readyz`)
- Metrics & tracing
- CI/CD pipelines

Happy hacking with Rust 🚀
