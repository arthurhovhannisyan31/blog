use dioxus::prelude::*;

use crate::configs::route::Route;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    div {
      id: "navbar",
      button {
        id: "nav-button",
        onclick: move |_| {
          info!("Home button")
        },
        "Home",
      }
      Link {
          to: Route::Home {},
          "Home"
      }
      Link {
          to: Route::Login,
          onclick: |_| {
            //
          },
          "Logout"
      }
    }
  }
}
