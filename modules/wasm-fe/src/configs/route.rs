use dioxus::prelude::*;

use crate::components::layout::Layout;
use crate::view::{
  create_post::CreatePost, edit_post::EditPost, home::Home, login::Login,
  register::Register,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home,
    #[route("/posts/:id")]
    EditPost { id: i64 },
    #[route("/posts/new")]
    CreatePost,
    #[route("/login")]
    Login,
    #[route("/register")]
    Register,
}
