FROM rust:1.95 AS build-server
SHELL ["/bin/bash", "-c"]
# copy locked dependencies
COPY --from=builder /usr/local/cargo /usr/local/cargo

# setup missing system dependencies
RUN apt-get update
RUN apt-get install protobuf-compiler -y

WORKDIR /home/blog
COPY --from=builder /home/blog .
# force sqlx to use cached queries metadata
ENV SQLX_OFFLINE=true
RUN cargo build --release -p blog-server

# glibc compatible container
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y curl \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /home/blog
COPY --from=build-server /home/blog/target/release/blog-server .
CMD ["/home/blog/blog-server"]