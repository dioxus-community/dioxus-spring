use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_spring::{use_animated, use_spring};
use dioxus_use_mounted::use_mounted;
use log::LevelFilter;
use std::time::Duration;

fn app() -> Element {
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

    log::info!("render!");

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
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch::launch_cfg(app, Default::default())
}
