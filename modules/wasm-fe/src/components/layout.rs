use dioxus::prelude::*;

use crate::components::nav::Navbar;
use crate::configs::route::Route;

#[component]
pub fn Layout() -> Element {
  rsx! {
    div {
      id: "layout",
      Navbar {}
      div {
        id: "layout-container",
        div {
          id:  "layout-content",
          Outlet::<Route> {}
        }
      }
    }
  }
}
