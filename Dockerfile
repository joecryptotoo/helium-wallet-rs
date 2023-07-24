FROM rust:1.71.0

WORKDIR /usr/src/helium-wallet-rs

RUN apt-get update && \
    apt-get install -y protobuf-compiler

COPY . .

RUN cargo build

ENTRYPOINT /usr/src/helium-wallet-rs/target/debug/helium-wallet
