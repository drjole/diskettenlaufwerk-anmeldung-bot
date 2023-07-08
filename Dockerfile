FROM rust:1.67-alpine as builder
WORKDIR /usr/src/myapp
COPY Cargo.toml .
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build
RUN rm src/lib.rs
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]
