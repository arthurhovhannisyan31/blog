<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>blog-fe</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a> <a href="https://dioxuslabs.com/"><img height="14px" alt="Dioxus" src="https://dioxuslabs.com/assets/smalllogo-dxh2bc42f6c273a797.png"/></a></h4>
</div>


[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/blog/actions/workflows/packages-validation.yml)

## Overview

This is the `blog front-end` implementation build with the [Dioxus](https://dioxuslabs.com/) framework. The application
supports regular user authentication flow and posts CRUD operations.
The application stores the authentication token in the client persistent storage (local-storage for browser) and
restores token on page reload.

## Description

Please look at supported pages in [route config](./src/configs/route.rs).
All pages wrapped in [layout](./src/components/layout.rs) component with basic navigation and user authentication
controls.
The application provides [shared state](./src/infrastructure/state.rs)
using [context api](https://dioxuslabs.com/learn/0.7/essentials/basics/context). The state includes information about
authorized user, its token and ID, the persistent storage reference, and `HTTP` client.
The application does not share the same domain instance types with the rest of the workspace,
hence [types are duplicated](./src/infrastructure/model.rs).
All the API calls are implemented in the [api client](./src/infrastructure/client.rs).

## Usage

Please make sure to provide a `.env` variables file, since the application does use a `.env` file of the workspaces.
Please run the following command in the root of this crate. You can provide your own `PORT` variable with
`dx serve --port <PORT>` flag.

```shell
dx serve
```

## Stack

- [Rust](https://rust-lang.org/)
- [Dioxus](https://dioxuslabs.com/)
- [Reqwest](https://docs.rs/reqwest/latest/reqwest/)
- [Serde](https://docs.rs/serde/latest/serde/)
- [Thiserror](https://docs.rs/thiserror/latest/thiserror/)

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of your options.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT