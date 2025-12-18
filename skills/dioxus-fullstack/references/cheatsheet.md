# Dioxus 0.7 Quick Reference

## DX CLI Commands
```bash
# Create project
dx create <name> --template <fullstack|desktop|mobile|web>

# Development
dx serve
dx check

# Dependencies (preferred over manual Cargo.toml)
dx add <crate> --features "feature1,feature2"
dx remove <crate>

# Build
dx build --platform <web|desktop|mobile>
dx build --release

# Testing
dx test
dx test --release
```

## Component Syntax
```rust
use dioxus::prelude::*;

#[component]
fn ComponentName(cx: Scope, props: PropsType) -> Element {
    render! {
        div { "Hello" }
    }
}
```

## Hooks
- `use_signal(cx, || initial)` - Local state
- `use_resource(cx, || async move { ... })` - Async data
- `use_memo(cx, (deps,), |(deps,)| compute(deps))` - Computed values
- `use_effect(cx, (deps,), |(deps,)| side_effect())` - Side effects
- `use_coroutine(cx, |rx| async move { ... })` - State machines

## Server Functions
```rust
#[server]
async fn function_name(params: Type) -> Result<Return, ServerFnError> {
    // Server-only code
}
```

## Routing
```rust
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/path/:param")]
    Path { param: String },
    #[nest("/admin")]
        #[route("/dashboard")]
        AdminDashboard {},
    #[end_nest]
}
```

## Common Patterns
- Signal reactivity: `let value = signal.read();`
- Signal mutation: `signal.write().push(new_val);`
- Event handlers: `onclick: move |_| handler()`
- Conditional rendering: `if condition { render! { ... } }`
- Navigation: `use_navigator(cx).push(Route::Home {})`