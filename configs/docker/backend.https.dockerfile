FROM rust:1.95 AS build-backend
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
RUN cargo build --release -p blog-server --features "tls"

# glibc compatible container
FROM debian:trixie-slim
RUN apt-get update \
    && apt-get install -y curl \
    && apt-get install -y bash \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /home/blog
# only required for secure connections: BACKEND_TLS=true
COPY --from=build-backend /home/blog/configs/nginx/certs configs/nginx/certs
COPY --from=build-backend /home/blog/configs/scripts/backend-healthcheck.sh configs/scripts/backend-healthcheck.sh
COPY --from=build-backend /home/blog/target/release/blog-server .
ENTRYPOINT ["/home/blog/blog-server"]