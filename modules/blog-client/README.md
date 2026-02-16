<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>blog-client</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>


[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml)

## Overview

This crate provides simple access to all server APIs, implementing `HTTP` and `gRPC` protocols access through a single
entry point.

## Description

Both, `HTTP` and `gRPC` clients implement the [AbstractBlogClient](./src/lib.rs) trait and can be used instead of each
other.
The [HTTP client](./src/http_client.rs) uses [reqwest](https://docs.rs/reqwest/latest/reqwest/) as transport
implementation, the [gRPC client](./src/grpc_client.rs)
uses generated client from [proto_generator](../proto-generator/README.md) crate.
Client builder provides `Transport` enum for building `HTTP` or `gRPC` server address.
Address should be compatible with [SocketAddr](https://doc.rust-lang.org/beta/std/net/enum.SocketAddr.html) trait.

## Usage

Please add the `blog-client` as a dependency to your project:

```
// Cargo.toml

blog-client = { path = "./modules/blog-client" }
```

```rust
use blog_client::{Transport, client::BlogClient};

fn main() -> anyhow::Error<BlogClient> {
  let transport = Transport::Http("http_addr");

  let client = BlogClient::new(transport).await?;
}
```

Please take a look at [examples](./examples) for details of usage. You can run them from `blog-client` crate root:

```shell
cargo run --example blog-client-http
cargo run --example blog-client-grpc
```

## Stack

- [Rust](https://rust-lang.org/)
- [Reqwest](https://docs.rs/reqwest/latest/reqwest/)
- [Thiserror](https://docs.rs/thiserror/latest/thiserror/)
- [Tonic](https://docs.rs/tonic/latest/tonic/)
- [Tracing](https://docs.rs/tracing/latest/tracing/)

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of your options.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT
