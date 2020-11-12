FROM arm64v8/rust:latest AS builder

RUN apt update && apt upgrade -y
RUN rustup target add aarch64-unknown-linux-musl

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./
RUN rm -rf src

RUN USER=root cargo new --bin server
RUN USER=root cargo new --lib wasm

RUN rm ./server/src/*.rs
RUN rm ./wasm/src/*.rs

COPY ./server/Cargo.toml ./server
COPY ./server/src/main.rs ./server/src
COPY ./wasm/Cargo.toml ./wasm
RUN mkdir ./wasm/textures
COPY ./wasm/textures ./wasm/textures
COPY ./wasm/src/lib.rs ./wasm/src
COPY ./static ./static

ENV CC=aarch64-linux-gnu-gcc
ENV TARGET_CC=aarch64-linux-gnu-gcc
ENV TARGET_AR=aarch64-linux-gnu-gcc-ar

RUN cargo rustc --release --package server --target aarch64-unknown-linux-musl -- -C link-arg=-lgcc
RUN strip --strip-all target/aarch64-unknown-linux-musl/release/server

FROM arm64v8/busybox:latest AS runtime
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/server ./
COPY --from=builder /etc/ssl/certs /etc/ssl/certs
RUN mkdir static
COPY --from=builder /app/static ./static

ENTRYPOINT [ "./server" ]
EXPOSE 9898/tcp
