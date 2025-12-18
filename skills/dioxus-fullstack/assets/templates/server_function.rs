// Dioxus 0.7 Server Function Template
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseData {
    pub id: u32,
    pub message: String,
}

#[server]
pub async fn server_function(param: String) -> Result<ResponseData, ServerFnError> {
    // Server-side logic here
    // Database queries, API calls, etc.

    let response = ResponseData {
        id: 1,
        message: format!("Processed: {}", param),
    };

    Ok(response)
}

#[component]
pub fn ServerComponent(cx: Scope) -> Element {
    let mut input = use_signal(cx, || "".to_string());
    let result = use_resource(cx, || async move {
        if !input.read().is_empty() {
            server_function(input.read().clone()).await
        } else {
            Ok(ResponseData {
                id: 0,
                message: "Enter input".to_string(),
            })
        }
    });

    render! {
        div { class: "server-component",
            input {
                value: "{input}",
                oninput: move |e| input.set(e.value()),
                placeholder: "Enter data"
            }

            div {
                match result.read().as_ref() {
                    Some(Ok(data)) => rsx! {
                        p { "ID: {data.id}" }
                        p { "Message: {data.message}" }
                    },
                    Some(Err(e)) => rsx! { p { "Error: {e}" } },
                    None => rsx! { p { "Loading..." } },
                }
            }
        }
    }
}