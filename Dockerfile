# ── Stage 1: Build frontend (Leptos/WASM via Trunk) ────────────────────────
FROM rust:1.78-slim AS frontend-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev curl nodejs npm ca-certificates && \
    rustup target add wasm32-unknown-unknown && \
    cargo install trunk && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared/ shared/
COPY frontend/ frontend/

WORKDIR /app/frontend
RUN trunk build --release

# ── Stage 2: Build backend ─────────────────────────────────────────────────
FROM rust:1.78-slim AS backend-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared/ shared/
COPY backend/ backend/
# Bring in the compiled frontend so Trunk paths resolve (not needed for backend build).

# Use offline mode so sqlx doesn't need a live DB during compile.
ENV SQLX_OFFLINE=true

RUN cargo build --release -p backend

# ── Stage 3: Runtime image ─────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=backend-builder  /app/target/release/backend    ./backend
COPY --from=frontend-builder /app/frontend/dist             ./frontend/dist
COPY migrations/                                            ./migrations/

ENV FRONTEND_DIST=/app/frontend/dist
ENV PORT=8080

EXPOSE 8080

ENTRYPOINT ["./backend"]
