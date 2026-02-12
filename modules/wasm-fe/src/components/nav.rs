use dioxus::prelude::*;

use crate::store::state::AppState;

#[component]
pub fn Navbar() -> Element {
  let user_data = consume_context::<AppState>().user;
  let is_authenticated = user_data().is_some();

  rsx! {
    div {
      id: "navbar",
      button {
        id: "my-button",
        onclick: move |_| {
          info!("Home button")
        },
        "Home",
      }
      button {
        id: "my-button",
        onclick: move |_| {
          info!("Home button")
        },
        if is_authenticated {"Logout"} else {"Login"}
      }
    }
  }
}
