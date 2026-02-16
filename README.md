<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>blog</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>


[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml)

## Overview

This is a blog workspace which
includes [back-end](./modules/blog-server/README.md), [front-end](./modules/blog-fe/README.md), [cli tool](./modules/blog-cli/README.md)
and set of utility crates.

Workspace crates supports registration and authentication of users using jwt tokens, full CRUD operation on posts.

## Description

- [Server](./modules/blog-server/README.md) is implemented with [Actix](https://actix.rs/) framework with basic
  authentication user-flow and `CRUD` API for blog posts.
  Server supports `HTTP` and `gRPC` protocols with authentication middleware. Server starts both(`HTTP` and `gRPC`)
  servers simultaneously.
  The `gRPC` server supports reflection API which is very handy using cli tools like `grpcurl`.
  Server uses single [postgres](https://www.postgresql.org/) database for implemented protocols.


- [Front-end](./modules/blog-fe/README.md) is build with [Dioxus](https://dioxuslabs.com/) framework. The application
  supports regular user authentication flow and posts CRUD operations. Application stores authentication token in client
  persistent storage (local-storage for browser) and restores token on page reload.


- [Cli](./modules/blog-cli/README.md) is a binary implementation of [blog-client](./modules/blog-client/README.md) crate
  and provides simple access to all server APIs, implementing `HTTP` and `gRPC` protocols.
  Cli supports arguments validation and provides help output.


- [Client](./modules/blog-client/README.md) provides simple access to all server APIs, implementing `HTTP` and `gRPC`
  protocols access through single entry point.
  Crate provides implementation for `HTTP` and `gRPC` clients which can be used with `cli builder`
  The [HTTP client](./src/http_client.rs) uses [reqwest](https://docs.rs/reqwest/latest/reqwest/) as transport
  implementation, the [gRPC client](./src/grpc_client.rs)
  uses generated client from [proto_generator](../proto-generator/README.md) crate.
  Client builder provides `Transport` enum for building `HTTP` or `gRPC` server address.


- [Proto-generator](./modules/proto-generator/README.md) is a utility crate which provides generated idiomatic `Rust`
  code for `gRPC` services declared
  in [Protobuf schema](./proto/blog.proto). Crate includes: `server` and `client` traits, `message`s types and
  `reflection api` descriptors.


- [Common](./modules/proto-generator/README.md) is crate which includes modules reused in workspace crates.

## Usage

Current release does not support containerized application as `docker` hence project needs to build manually.

Please ensure local `postgres` database as up and running, the `.env` file is populated in the workspace root and then
run `make build`.
Run `blog-server` and `blog-cli` binaries from `target/release` folder.

> In current release `front-end` is not included in workspace build process hence you need to run `dx serve` command
> from 'blog-fe' crate root.

> You can use pre-build binaries with default environment variables from
> the [GH Releases](https://github.com/arthurhovhannisyan31/blog/releases).
> Download the archived binaries for your OS and use the `blog-server` from `target/release` folder.


Please see [Server](./modules/blog-server/README.md), [Front-end](./modules/blog-fe/README.md)
and [Cli](./modules/blog-cli/README.md) documentation for details.

## Stack

- [Rust](https://rust-lang.org/)
- [Actix](https://docs.rs/actix/latest/actix/)
- [Clap](https://docs.rs/clap/latest/clap/)
- [Dioxus](https://dioxuslabs.com/)
- [Reqwest](https://docs.rs/reqwest/latest/reqwest/)
- [Serde](https://docs.rs/serde/latest/serde/)
- [Thiserror](https://docs.rs/thiserror/latest/thiserror/)
- [Tokio](https://docs.rs/tokio/latest/tokio/)
- [Tonic](https://docs.rs/tonic/latest/tonic/)
- [Tracing](https://docs.rs/tracing/latest/tracing/)

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of at your option.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT
