use dioxus::prelude::*;

use crate::components::posts_list::PostsList;
use crate::store::state::AppState;

#[component]
pub fn Home() -> Element {
  let user_data = consume_context::<AppState>().user;
  let is_authenticated = user_data().is_some();

  rsx! {
    div {
      id: "home",
      PostsList {},
      button {
        id: "create-post",
        disabled: !is_authenticated,
        title: if is_authenticated {""} else {"Please log in to crate posts"},
        onclick: move |_| {
          //
        },
        "+"
      }
    }
  }
}
