# dioxus-spring

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/dioxus-community/dioxus-spring#license)
[![Crates.io](https://img.shields.io/crates/v/dioxus-spring.svg)](https://crates.io/crates/dioxus-spring)
[![Docs](https://docs.rs/dioxus-spring/badge.svg)](https://docs.rs/dioxus-spring/latest/dioxus_spring/)
[![CI](https://github.com/dioxus-community/dioxus-spring/workflows/CI/badge.svg)](https://github.com/dioxus-community/dioxus-spring/actions)

Animation library for [Dioxus](https://dioxuslabs.com).

Pairs great with [dioxus-use-gesture](https://github.com/dioxus-community/dioxus-use-gesture)!



```rust
let container_ref = use_mounted();
let rect = use_size(container_ref);

let mut is_big = use_signal(|| false);
let spring = use_spring(
    if is_big() { rect.width() as f32 } else { 0f32 },
    Duration::from_millis(500),
);

let animated_ref = use_mounted();
use_animated(animated_ref, spring, |width| {
    format!(
        r"
        width: {width}px;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
        background: #27ae60;
    "
    )
});

rsx!(
    div {
        position: "relative",
        width: "200px",
        height: "50px",
        border: "2px solid #eee",
        onmounted: move |event| container_ref.onmounted(event),
        onclick: move |_| is_big.set(!is_big()),
        div { onmounted: move |event| animated_ref.onmounted(event) }
        span {
            position: "absolute",
            top: "50%",
            left: "50%",
            transform: " translate(-50%, -50%)",
            z_index: 9,
            "Click me!"
        }
    }
)
```
