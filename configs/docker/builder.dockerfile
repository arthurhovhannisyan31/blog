FROM rust:1.95 AS builder
WORKDIR /home/blog
COPY . .
# $CARGO_HOME=/usr/local/cargo/
RUN cargo fetch --locked