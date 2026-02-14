use dioxus::prelude::*;
use dioxus_sdk_storage::{LocalStorage, use_synced_storage};

mod components;
mod configs;
mod generated;
mod infrastructure;
mod view;

use crate::configs::assets::assets;
use crate::configs::route::Route;
use crate::infrastructure::client::BlogClient;
use crate::infrastructure::state::{AppState, UserData};

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  let api_base_url =
    format!("http://{}:{}/api", generated::HOST, generated::PORT);
  let client = use_signal(|| {
    BlogClient::new(api_base_url).expect("Failed to build api client")
  });
  let mut auth = use_signal(|| None);
  let storage =
    use_synced_storage::<LocalStorage, String>("user_data".to_string(), || {
      "".into()
    });

  use_context_provider(|| AppState {
    auth,
    storage,
    client,
  });
  use_effect(move || {
    if !storage().is_empty() {
      if let Ok(user_data) = serde_json::from_str::<UserData>(&storage()) {
        auth.set(Some(user_data));
      }
    }
  });

  rsx! {
      document::Link { rel: "icon", href: assets::FAVICON }
      document::Link { rel: "stylesheet", href: assets::MAIN_CSS }
      Router::<Route> {}
  }
}
