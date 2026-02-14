use crate::configs::route::Route;
use crate::infrastructure::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
  let navigator = use_navigator();
  let mut context = consume_context::<AppState>();
  let is_authenticated = context.auth.read().is_some();

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
      if is_authenticated {
        button {
          id: "my-button",
          onclick: move |_| {
            context.auth.set(None);
            context.storage.set("".to_string());
            navigator.push(Route::Login{});
          },
          "Logout"
        }
      } else {
        button {
          id: "my-button",
          onclick: move |_| {
            navigator.push(Route::Login{});
          },
          "Login"
        }
      }
    }
  }
}
