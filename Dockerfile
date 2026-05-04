# ─── Stage 1: Build frontend ──────────────────────────────────────
FROM node:22-alpine AS frontend-builder

RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app/frontend

COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY frontend/ ./
RUN pnpm run build

# ─── Stage 2: Build backend ────────────────────────────────────────
FROM rust:1-alpine AS backend-builder

RUN apk add --no-cache musl-dev curl

WORKDIR /app/backend

# Cache dependencies
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY backend/src/ ./src/
# Force rebuild with real source
RUN touch src/main.rs && cargo build --release

# ─── Stage 3: Runtime ──────────────────────────────────────────────
FROM alpine:3

RUN apk add --no-cache ca-certificates

COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist
COPY --from=backend-builder /app/backend/target/release/campus-trade /app/campus-trade

ENV FRONTEND_DIR=/app/frontend/dist
EXPOSE 3000

CMD ["/app/campus-trade"]
