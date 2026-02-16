<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>blog-cli</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>


[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml)

## Overview

The `blog cli` is a binary implementation of the `blog-client` crate and provides simple access to all server APIs,
implementing `HTTP` and `gRPC` protocols.

## Description

Please run `blog-cli -h` for a detailed list of supported commands and nested commands.
The `blog cli` supports argument validation:

- `server` value should be compatible with [SocketAddr](https://doc.rust-lang.org/beta/std/net/enum.SocketAddr.html)
  trait.
- `username`, `email`, `password`, and post `title` fields should be non-empty strings
- post `content` though might be missing
- pagination `limit` and `offset` arguments are optional

The `blog cli` stores authorization token in a local file `.blog_token` hence make sure the binary has sufficient rights
for
file-system access.

## Usage

Please find the latest build binary for `cli` in
the [GH Releases](https://github.com/arthurhovhannisyan31/blog/releases).
Download the archived binary for your OS and run it from the `target/release` folder.

You can use the following list of commands as example. Please provide the `blog-cli --grpc` flag to use the `gRPC`
protocol.

```shell
blog-cli register --username "ivan" --email "ivan@example.com" --password "secret123".
blog-cli login --username "ivan" --password "secret123".
blog-cli create --title "Мой первый пост" --content "Содержание".
blog-cli create --title "Мой первый пост" --content "Содержание"
blog-cli get --id 1.
blog-cli update --id 1 --title "Обновлённый заголовок".
blog-cli delete --id 1.
blog-cli list --limit 20 --offset 0.
```

## Stack

- [Rust](https://rust-lang.org/)
- [Clap](https://docs.rs/clap/latest/clap/)
- [Serde](https://docs.rs/serde/latest/serde/)
- [Thiserror](https://docs.rs/thiserror/latest/thiserror/)
- [Tokio](https://docs.rs/tokio/latest/tokio/)
- [Tracing](https://docs.rs/tracing/latest/tracing/)

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of your options.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT
