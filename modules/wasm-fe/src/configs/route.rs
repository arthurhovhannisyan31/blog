use dioxus::prelude::*;

use crate::components::layout::Layout;
use crate::view::{home::Home, login::Login, post::Post, register::Register};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/post/:id")]
    Post { id: i64 },
    #[route("/login")]
    Login,
    #[route("/register")]
    Register,
}
