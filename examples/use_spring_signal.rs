use dioxus::prelude::*;
use dioxus_spring::use_spring_signal;
use log::LevelFilter;
use std::time::Duration;

fn app() -> Element {
    let (value, value_spring) = use_spring_signal(0f32);

    use_hook(move || {
        value_spring.animate(1., Duration::from_secs(1));
    });

    use_memo(move || {
        log::info!("{}", value());
    });

    rsx!()
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus::launch(app)
}
