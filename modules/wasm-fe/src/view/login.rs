use crate::configs::route::Route;
use crate::infrastructure::model::{AuthRequest, AuthResponse};
use crate::infrastructure::state::{AppState, UserData};
use dioxus::prelude::*;
use reqwest::Client;
use serde_json::json;

#[component]
pub fn Login() -> Element {
  let navigator = use_navigator();
  let mut context = consume_context::<AppState>();
  let mut email = use_signal(|| "".to_string());
  let mut password = use_signal(|| "".to_string());
  let mut error = use_signal(|| false);

  let handle_login = move |_| async move {
    if email.read().is_empty() || password.read().is_empty() {
      error.set(true);
      return;
    }

    let auth = login(email.read().to_string(), password.read().to_string())
      .await
      .unwrap();

    let user_data = UserData {
      token: format!("Bearer {}", auth.token),
      user_id: auth.user.user_id,
    };

    context.auth.set(Some(user_data.clone()));
    context.storage.set(json!(user_data).to_string());
    navigator.push(Route::Home {});
  };

  rsx! {
    div {
      id: "register",
      class: "container",
      div {
        id: "register-content",
        if error() {
          span {
            "Make sure all fields are filled and try again",
          }
        }
        label {
          id: "form-label",
          "Email",
          input {
            id: "register-email",
            type: "email",
            placeholder: "Email",
            autocomplete: "email",
            value: "{email}",
            oninput: move |event| email.set(event.value())
          }
        }
        label {
          id: "form-label",
          "Password",
          input {
            id: "register-password",
            placeholder: "password",
            autocomplete: "new-password",
            value: "{password}",
            oninput: move |event| password.set(event.value())
          }
        }
        button {
          id: "register-button",
          onclick: handle_login,
          "Login",
        }
        button {
          onclick: move |_| {
            email.set("".into());
            password.set("".into());
          },
          "Clear"
        }
        button {
          onclick: move |_| {
            navigator.push(Route::Register {});
          },
          "Create account"
        }
      }
    }
  }
}

async fn login(
  email: String,
  password: String,
) -> anyhow::Result<AuthResponse> {
  let client = Client::builder()
    .user_agent("user-Agent: wasm-fe")
    .build()?;
  let response = client
    .post("http://localhost:8080/api/v0/auth/login")
    .json(&AuthRequest { email, password })
    .send()
    .await?;
  let auth = response.json::<AuthResponse>().await?;

  Ok(auth)
}
