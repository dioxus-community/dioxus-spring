use dioxus::prelude::*;
use dioxus_spring::{use_spring_style, use_spring_signal};
use log::LevelFilter;
use std::time::Duration;

fn app(cx: Scope) -> Element {
    let spring_ref = use_spring_signal(cx, 1f32, |scale| {
        log::info!("{}", scale);
    });

    log::info!("render");

    render!(
        h1 {
            onmouseenter: move |_| spring_ref.animate(2., Duration::from_secs(1)),
            onmouseleave: move |_| spring_ref.animate(1., Duration::from_secs(1)),
            "Hover me!"
        }
    )
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
