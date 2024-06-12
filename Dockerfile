FROM rust:1.78.0-slim-bookworm AS base
WORKDIR /app

FROM base AS builder
RUN cargo new --bin foodfy-rust-api
COPY . /app
RUN cargo build \
    --release \
    -p foodfy-rust-api

FROM base
COPY --from=builder /app/target/release/foodfy-rust-api /app
CMD ["/app/foodfy-rust-api"]
