use dioxus::{
    hooks::use_memo,
    signals::{Readable, Signal},
    web::WebEventExt,
};
use dioxus_use_mounted::UseMounted;

/// Hook to use an animated value and apply it to a mounted element.
pub fn use_animated<V>(
    mounted: UseMounted,
    value_ref: Signal<V>,
    mut make_style: impl FnMut(V) -> String + 'static,
) where
    V: Clone,
{
    use_memo(move || {
        let value = value_ref.read();
        set_style(mounted, &make_style(value.clone()));
    });
}

fn set_style(mounted: UseMounted, style: &str) {
    if let Some(element) = &*mounted.signal.read() {
        let raw_elem = element.try_as_web_event().unwrap();

        raw_elem.set_attribute("style", style).unwrap();
    }
}
