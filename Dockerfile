# when changing image be aware of GLIB version matching in build and running images
FROM rust:1.92-trixie AS builder

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src

RUN cargo build --release

FROM debian:trixie-slim

WORKDIR /app
ENV PATH="$PATH:/app"

COPY --from=builder /app/target/release/lunch-messenger /app/

ENTRYPOINT [ "/app/lunch-messenger" ]

