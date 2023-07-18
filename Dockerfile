ARG BINARY=bot

FROM lukemathwalker/cargo-chef:latest-rust-1.71 as chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG BINARY
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --all-targets

FROM debian:bullseye-slim
ARG BINARY
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/${BINARY} /usr/local/bin/${BINARY}
RUN chmod +x /usr/local/bin/${BINARY}

# https://github.com/moby/moby/issues/18492
ENV BINARY=${BINARY}
ENV RUST_LOG=INFO

CMD "$BINARY"
