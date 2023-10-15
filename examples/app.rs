use dioxus::prelude::*;
use dioxus_spring::use_spring_style;
use std::time::Duration;

fn app(cx: Scope) -> Element {
    let spring_ref = use_spring_style(cx, 10f32, 100f32, Duration::from_secs(1), |font_size| {
        format!("font-size: {font_size}px;")
    });

    render!(
        h1 {
            onmounted: move |event| {
                spring_ref.mount(event.data);
            },
            onclick: move |_| {
                spring_ref.start();
            },
            "Click me!"
        }
    )
}

fn main() {
    dioxus_web::launch(app)
}
