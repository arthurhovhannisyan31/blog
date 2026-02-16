<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>proto-generator</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>


[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml)

## Overview

This crate includes generated idiomatic `Rust` code for `gRPC` services declared
in [Protobuf schema](./proto/blog.proto).

## Description

Crate includes: `server` and `client` traits, `message` types and `reflection api` descriptors.
All generated `message` structs implement `serde::Serialize, serde::Deserialize` traits.

## Usage

Add the `proto-generator` to your crate dependencies and use required modules.

```
// Cargo.toml

proto-generator = { path = "./modules/proto-generator" }
```

```rust
  use proto_generator::blog::*;
```

Use `reflection api` descriptors as follows:

```rust
use tonic_reflection::server::Builder;
use proto_generator::blog::FILE_DESCRIPTOR;
use tonic::transport::{Error, Server};

fn main() {
  let grpc_reflection_service = Builder::configure()
    .register_encoded_file_descriptor_set(FILE_DESCRIPTOR)
    .build_v1()
    .expect("Failed building grpc reflection service");

  let grpc_server = Server::builder()
    .add_service(grpc_reflection_service)
    .serve("addr");
}

```

## Stack

- [Rust](https://rust-lang.org/)
- [Tonic](https://docs.rs/tonic/latest/tonic/)
- [Tonic prost build](https://docs.rs/tonic-prost-build/latest/tonic_prost_build/)
- [Serde](https://docs.rs/serde/latest/serde/)

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of your options.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT
