ARG BINARY=bot

FROM rust:1.71 as builder
ARG BINARY
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin ${BINARY}

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
