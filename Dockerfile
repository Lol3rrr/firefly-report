FROM rust:1.87 AS builder

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/firefly-report"]
