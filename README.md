<div align="center">
<h1>Dioxus Spring</h1>
 <a href="https://crates.io/crates/dioxus-spring">
    <img src="https://img.shields.io/crates/v/dioxus-spring?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/dioxus-spring/latest/dioxus-spring">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
   <a href="https://github.com/matthunz/dioxus-spring/actions">
    <img src="https://github.com/matthunz/dioxus-spring/actions/workflows/ci.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
 <a href="https://github.com/matthunz/dioxus-spring/tree/main/examples">Examples</a>
</div>

<br>

```rust
let spring_ref = use_spring_style(cx, 1f32, |scale| {
    format!("transform-origin: top left; transform: scale({scale});")
});

render!(
    h1 {
        onmounted: move |event| {
            spring_ref.mount(event.data);
        },
        onmouseenter: move |_| {
            spring_ref.transition_to(2., Duration::from_secs(1));
        },
        onmouseleave: move |_| {
            spring_ref.transition_to(1., Duration::from_secs(1));
        },
        "Hover me!"
    }
)
```
