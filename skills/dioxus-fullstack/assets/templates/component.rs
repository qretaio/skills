// Dioxus 0.7 Component Template
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ComponentProps {
    #[props(default = "".to_string())]
    pub title: String,
    pub children: Element,
}

#[component]
pub fn Component(cx: Scope<ComponentProps>) -> Element {
    render! {
        div { class: "component",
            h2 { "{cx.props.title}" }
            {cx.props.children}
        }
    }
}