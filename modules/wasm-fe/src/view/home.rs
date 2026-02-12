use dioxus::prelude::*;

use crate::components::posts_list::PostsList;

#[component]
pub fn Home() -> Element {
  rsx! {
      PostsList {}
  }
}
