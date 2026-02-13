use dioxus::prelude::*;

use crate::components::posts_list::PostsList;
use crate::configs::route::Route;
use crate::infrastructure::state::AppState;

#[component]
pub fn Home() -> Element {
  let navigator = use_navigator();
  let auth_data = consume_context::<AppState>().auth;
  let is_authenticated = auth_data().is_some();

  rsx! {
    div {
      id: "home",
      class: "container",
      PostsList {},
      button {
        id: "create-post",
        disabled: !is_authenticated,
        title: if is_authenticated {"Create a post"} else {"Please log in to crate posts"},
        onclick: move |_| {
          navigator.push(Route::CreatePost {});
        },
        "+"
      }
    }
  }
}
