FROM rust:1.62-alpine3.16

RUN apk add alpine-sdk

RUN cargo install cargo-watch

RUN apk add protoc

RUN apk add cmake

WORKDIR /root