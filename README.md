# dioxus-spring

```rust
let spring_ref = use_spring(cx, 10, 100, |font_size| {
    format!("font-size: {font_size}px;")
});

render!(h1 {
    onmounted: move |event| {
        spring_ref.mount(event.data);
    },
    "Hello World!"
})
```
