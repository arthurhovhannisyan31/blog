use crate::components::post_edit::PostEdit;
use dioxus::prelude::*;

#[component]
pub fn Post(id: i64) -> Element {
  rsx! {
    div {
      id: "post",
      PostEdit { id }
    }
  }
}
