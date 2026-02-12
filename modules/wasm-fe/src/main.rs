use dioxus::prelude::*;

mod components;
mod configs;
mod store;
mod view;

use crate::configs::assets::assets;
use crate::configs::route::Route;
use crate::store::state::AppState;

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  let user = use_signal(|| None);
  use_context_provider(|| AppState { user });

  rsx! {
      document::Link { rel: "icon", href: assets::FAVICON }
      document::Link { rel: "stylesheet", href: assets::MAIN_CSS }
      Router::<Route> {}
  }
}
