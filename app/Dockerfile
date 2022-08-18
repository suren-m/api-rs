# Step 1: Compute a recipe file
FROM rust:1.63-bullseye as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Step 2: Cache project dependencies
FROM rust:1.63-bullseye as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Step 3: Build the binary
FROM rust:1.63-bullseye as builder
WORKDIR app
COPY . .
# Copy over the cached dependencies from above
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin api-server

# Step 4:
# Create a tiny output image.
FROM debian:bullseye-slim as runtime
WORKDIR app
RUN apt update -y && apt install -y libpq-dev
COPY --from=builder /app/target/release/api-server /usr/local/bin
EXPOSE 8000
CMD ["api-server"]

# # Builder
# FROM rust:1.55.0-bullseye as build

# WORKDIR /api-rs
# COPY . .

# RUN cargo build --release

# # The final image
# FROM debian:buster-slim

# # Needed for Diesel Postgres migrations
# RUN apt update -y && apt install -y libpq-dev

# RUN mkdir -p /usr/local/bin
# COPY --from=build /api-rs/target/release/api-server /usr/local/bin

# EXPOSE 8000
# CMD ["api-server"]

