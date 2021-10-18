# Builder
FROM rust:1.55.0-bullseye as build

WORKDIR /api-rs
COPY . .

RUN cargo build --release

# The final image
FROM debian:buster-slim

# Needed for Diesel Postgres migrations
RUN apt update -y && apt install -y libpq-dev

RUN mkdir -p /usr/local/bin
COPY --from=build /api-rs/target/release/api-server /usr/local/bin

EXPOSE 8000
CMD ["api-server"]

