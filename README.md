[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/matthunz/dioxus-spring#license)
[![Crates.io](https://img.shields.io/crates/v/dioxus-spring.svg)](https://crates.io/crates/dioxus-spring)
[![Docs](https://docs.rs/dioxus-spring/badge.svg)](https://docs.rs/dioxus-spring/latest/dioxus-spring/)
[![CI](https://github.com/matthunz/dioxus-spring/workflows/CI/badge.svg)](https://github.com/matthunz/dioxus-spring/actions)

Animation library for [Dioxus](https://dioxuslabs.com).

Pairs great with [dioxus-use-gesture](https://github.com/matthunz/dioxus-use-gesture)!



```rust
let is_big = use_state(cx, || false);
let spring = use_spring(
    cx,
    if **is_big { 2f32 } else { 1f32 },
    Duration::from_millis(500),
);

let mounted = use_mounted(cx);
use_animated(cx, mounted, spring, |scale| {
    format!("transform-origin: top left; transform: scale({scale})")
});

render!(
    div {
        onmounted: move |event| mounted.onmounted(event),
        onclick: move |_| is_big.set(!is_big),
        "Click me!"
    }
)
```
