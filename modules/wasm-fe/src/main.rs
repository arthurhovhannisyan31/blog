use dioxus::prelude::*;

mod components;
mod configs;
mod view;

use crate::configs::assets::assets;
use crate::configs::route::Route;

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
      document::Link { rel: "icon", href: assets::FAVICON }
      document::Link { rel: "stylesheet", href: assets::MAIN_CSS }
      Router::<Route> {}
  }
}
