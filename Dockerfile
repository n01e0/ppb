FROM rust:latest AS builder
WORKDIR /build
COPY . /build
RUN apt-get update && apt-get install -yqq musl musl-tools pkg-config libssl-dev
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/ppb /ppb
ENTRYPOINT ["/ppb"]
