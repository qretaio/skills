// Dioxus 0.7 Route Template
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/blog/:id")]
    BlogPost { id: u32 },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
fn Home(cx: Scope) -> Element {
    render! {
        div { class: "home",
            h1 { "Welcome to Dioxus 0.7" }
            Link { to: Route::About {}, "About" }
            Link { to: Route::BlogPost { id: 1 }, "Blog Post 1" }
        }
    }
}

#[component]
fn About(cx: Scope) -> Element {
    render! {
        div { class: "about",
            h1 { "About" }
            Link { to: Route::Home {}, "Back to Home" }
        }
    }
}

#[component]
fn BlogPost(cx: Scope, id: u32) -> Element {
    render! {
        div { class: "blog-post",
            h1 { "Blog Post {id}" }
            p { "This is blog post number {id}" }
            Link { to: Route::Home {}, "Back to Home" }
        }
    }
}

#[component]
fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        div { class: "not-found",
            h1 { "404 - Not Found" }
            p { "Route: {route.join(\"/\")}" }
            Link { to: Route::Home {}, "Home" }
        }
    }
}

#[component]
pub fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}