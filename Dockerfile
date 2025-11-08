# https://depot.dev/blog/rust-dockerfile-best-practices
FROM rust:1.91 AS base
RUN cargo install cargo-chef

FROM base AS planner

WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY src src
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release


FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/app /usr/local/bin/app
CMD ["app"]
