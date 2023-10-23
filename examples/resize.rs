use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_spring::{use_animated, use_spring};
use dioxus_use_mounted::use_mounted;
use log::LevelFilter;
use std::time::Duration;

fn app(cx: Scope) -> Element {
    let container_ref = use_mounted(cx);
    let rect = use_size(cx, container_ref);

    let is_big = use_state(cx, || false);
    let spring = use_spring(
        cx,
        if **is_big { rect.width() as f32 } else { 0f32 },
        Duration::from_millis(500),
    );

    let animated_ref = use_mounted(cx);
    use_animated(cx, animated_ref, spring, |width| {
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

    log::info!("render!");

    render!(
        div {
            position: "relative",
            width: "200px",
            height: "50px",
            border: "2px solid #eee",
            onmounted: move |event| container_ref.onmounted(event),
            onclick: move |_| is_big.set(!is_big),
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
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
