FROM rust:latest as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY . .

RUN cargo build --release
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/my_app /app/my_app

EXPOSE 8080
CMD ["./my_app"]