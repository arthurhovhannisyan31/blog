use dioxus::prelude::*;
use reqwest::Client;

use crate::configs::route::Route;
use crate::store::model::{AuthRequest, AuthResponse};
use crate::store::state::{AppState, UserData};

#[component]
pub fn Login() -> Element {
  let navigator = use_navigator();
  let mut auth_data = consume_context::<AppState>().auth;

  let mut email = use_signal(|| "".to_string());
  let mut password = use_signal(|| "".to_string());
  let mut error = use_signal(|| false);

  let handle_register = move |_| async move {
    //
    if email.read().is_empty() || password.read().is_empty() {
      error.set(true);
      return;
    }

    let auth = login(email.read().to_string(), password.read().to_string())
      .await
      .unwrap();

    auth_data.set(Some(UserData {
      token: format!("Bearer {}", auth.token),
      user_id: auth.user.user_id,
    }));
    navigator.push(Route::Home {});
    // TODO Store token to local storage
    // Restore token from local storage
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
          onclick: handle_register,
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
