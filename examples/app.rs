use dioxus::prelude::*;
use dioxus_spring::use_spring_style;
use log::LevelFilter;
use std::time::Duration;

fn app(cx: Scope) -> Element {
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
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
