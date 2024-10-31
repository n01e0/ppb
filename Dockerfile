FROM rust:latest AS builder
WORKDIR /build
COPY . /build
RUN apt-get update && apt-get install -yqq pkg-config libssl-dev
RUN cargo build --release --target=x86_64-unknown-linux-gnu

FROM alpine:latest
COPY --from=builder /build/target/release/ppb /ppb
ENTRYPOINT ["/ppb"]
