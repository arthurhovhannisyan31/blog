FROM rust:1.95 AS build-web-client
SHELL ["/bin/bash", "-c"]
# copy locked dependencies
COPY --from=builder /usr/local/cargo /usr/local/cargo
# setup missing system dependencies
RUN curl -sSL https://dioxus.dev/install.sh | bash
RUN apt-get update
RUN apt-get install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  lld -y

WORKDIR /home/blog
COPY --from=builder /home/blog .

WORKDIR /home/blog/modules/blog-fe
# populate target folder
RUN cargo check -p blog-fe
RUN dx bundle --web --release

# web server
FROM nginx:1.25
COPY --from=build-web-client /home/blog/target/dx/blog-fe/release/web/public /usr/share/nginx/html