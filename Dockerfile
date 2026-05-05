FROM rust:1.82 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/mpg-smoke-01 /usr/local/bin/mpg-smoke-01
COPY --from=builder /app/migrations /app/migrations

EXPOSE 8080

ENV HOST=0.0.0.0
ENV PORT=8080

CMD ["/usr/local/bin/mpg-smoke-01"]
