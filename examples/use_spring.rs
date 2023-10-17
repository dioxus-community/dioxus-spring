use dioxus::prelude::*;
use dioxus_signals::use_signal;
use dioxus_spring::{use_animated, use_spring};
use log::LevelFilter;

fn app(cx: Scope) -> Element {
    let is_open = use_state(cx, || false);
    let spring = use_spring(cx, if **is_open { 2f32 } else { 1f32 });

    let element_ref = use_signal(cx, || None);
    use_animated(cx, element_ref, spring, |scale| {
        format!("transform-origin: top left; transform: scale({scale})")
    });

    log::info!("render!");

    render!(
        div {
            onmounted: move |event| element_ref.set(Some(event.data)),
            onclick: move |_| is_open.set(!is_open),
            "Click me!"
        }
    )
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
