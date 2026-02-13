use crate::configs::route::Route;
use crate::store::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
  let navigator = use_navigator();
  let mut auth_data = consume_context::<AppState>().auth;
  let is_authenticated = auth_data().is_some();

  rsx! {
    div {
      id: "navbar",
      button {
        id: "my-button",
        onclick: move |_| {
          navigator.push(Route::Home {});
        },
        "Home",
      }
      button {
        id: "my-button",
        onclick: move |_| {
          if is_authenticated {
            auth_data.set(None);
          }
          navigator.push(Route::Login{});
        },
        if is_authenticated {"Logout"} else {"Login"}
      }
    }
  }
}
