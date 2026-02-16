<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>blog-server</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>


[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml)

## Overview

This is the blog server implemented with [Actix](https://actix.rs/) framework.
Server implements basic authentication user-flow and `CRUD` API for blog posts.
Server supports `HTTP` and `gRPC` protocols with authentication middleware.

## Description

### Files structure and services

Project implements clean architecture files structure:

- [domain](./src/domain) models
- [data](./src/data) layer
- [application](./src/application) services
- [presentation](./src/presentation) layer

The presentation layer includes implementation of [HTTP](./src/presentation/http) and [gRPC](./src/presentation/grpc)
protocols.
Both protocols support authentication middleware. Server starts both(`HTTP` and `gRPC`) servers simultaneously.
Please learn more about available APIs in `HTTP`
services: [auth](./src/presentation/http/auth.rs), [posts](./src/presentation/http/posts.rs) and
`gRPC` [schema](../proto-generator/proto/blog.proto) and [service](./src/presentation/grpc/service.rs).

The `gRPC` server supports reflection API so you can request available services using bash script:

```shell
grpcurl -plaintext localhost:50051 list
```

### Database

Server uses single [postgres](https://www.postgresql.org/) database for implemented protocols. You can learn more about
data schema in [migrations](./migrations).
Server runs all migrations on start and tracks each migration using hash table, hence migration are applied only when
changed.

> Learn more:
> - [MigrationSource](https://docs.rs/sqlx/latest/sqlx/migrate/trait.MigrationSource.html)

### Security

#### Authentication and authorization

Both, `login` and `register` APIs provide `JWT` token in response.
The `register` API accepts `JSON` object with `username`, `email` and `password`.
The password is hashed using safe `argon2` algorithm and only hash of the password is saved to the database.
The `login` API accepts user email and password.
Provided password is hashed and compared against the users' password, found in database by provided email.

> Learn more:
> - [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/)
> - [argon2](https://docs.rs/argon2/latest/argon2/)

Some API routes are protected and require authentication header. The authorization header should implement `Bearer JWT`
format and included in request headers as `"Authorization": "Bearer {token}"`.

Server uses [jwt service](./src/infrastructure/jwt.rs) in implemented protocols' middlewares. The `jwt-service` decodes
provided token, extracts `user_id` value from decoded token and looks up for matching user
in [user repository](./src/data/user_repository.rs).

#### Cors

Server restricts access from unknown origins, hence origins white list should be set up in project env variables. Please
see `.env.template` for details. Only following request methods are allowed: `GET`, `POST`, `PUT`, `DELETE`, `OPTIONS`
with allowed headers as `Content-type` and `Authorization`.
Please see [cors configuration](./src/infrastructure/cors.rs) for details.

## Usage

### Setup

Please ensure local `postgres` database as up and running.
Create a database and use it in connection string as `DATABASE_URL` environment variable.

The `HOST`, `HTTP_PORT`, `GRPC_PORT` and `CORS_ORIGINS` variables are optional, please see default values
in [app-configs](./src/infrastructure/config.rs).
The `JWT_SECRET` and `DATABASE_URL` are required.

### Run

Please run `make build` from root of the project to build your version of binaries with provided `.env` file and run
`blog-server` from
`target/release` folder.

> You can use pre-build binaries with default environment variables from
> the [GH Releases](https://github.com/arthurhovhannisyan31/blog/releases).
> Download the archived binaries for your OS and use the `blog-server` from `target/release` folder.

```shell
blog-server
```

## Stack

- [Rust](https://rust-lang.org/)
- [Actix](https://docs.rs/actix/latest/actix/)
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
