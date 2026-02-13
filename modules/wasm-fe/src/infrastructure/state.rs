use dioxus::prelude::Signal;
use serde::{Deserialize, Serialize};

use crate::infrastructure::client::BlogClient;

#[derive(Clone, Debug)]
pub struct AppState {
  pub auth: Signal<Option<UserData>>,
  pub storage: Signal<String>,
  pub client: Signal<BlogClient>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserData {
  pub token: String,
  pub user_id: i64,
}
