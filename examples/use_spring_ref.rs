use dioxus::{logger::tracing::Level, prelude::*};
use dioxus_spring::use_spring_ref;
use std::time::Duration;

fn app() -> Element {
    let mut signal = use_signal(|| 0.);
    let spring_ref = use_spring_ref(0f32, move |x| signal.set(x));

    use_hook(move || {
        spring_ref.animate(1., Duration::from_secs(1));
    });

    use_memo(move || {
        log::info!("{}", signal());
    });

    rsx!()
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to init logger");

    dioxus::launch(app)
}
