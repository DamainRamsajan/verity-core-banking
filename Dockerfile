# Stage 1: Build
FROM rust:1.85-slim-bookworm AS builder
RUN apt-get update && apt-get install -y protobuf-compiler libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
RUN cargo build --release --bin verity

# Stage 2: Runtime
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /app/target/release/verity /usr/local/bin/verity
EXPOSE 8080
ENTRYPOINT ["verity"]
