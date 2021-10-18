FROM rust:1.55.0-bullseye

WORKDIR /usr/src/api-rs
COPY . .

RUN cargo install --path .

EXPOSE 8000
CMD ["api-server"]