FROM rust:1.87-bookworm AS build

WORKDIR /
RUN cargo new diskettenlaufwerk-anmeldung-bot
COPY Cargo.toml Cargo.lock /diskettenlaufwerk-anmeldung-bot/
WORKDIR /diskettenlaufwerk-anmeldung-bot
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

COPY . .
ENV SQLX_OFFLINE=true
RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  touch /diskettenlaufwerk-anmeldung-bot/src/main.rs
  cargo build --release
EOF

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates openssl && \
    rm -rf /var/lib/apt/lists/*
COPY --from=build /diskettenlaufwerk-anmeldung-bot/target/release/diskettenlaufwerk-anmeldung-bot /diskettenlaufwerk-anmeldung-bot
RUN chmod +x /diskettenlaufwerk-anmeldung-bot

CMD ["/diskettenlaufwerk-anmeldung-bot"]
