use dioxus::hooks::use_effect;
use dioxus_signals::{Readable, Signal};
use dioxus_use_mounted::UseMounted;

pub fn use_animated<V>(
    mounted: UseMounted,
    value_ref: Signal<V>,
    mut make_style: impl FnMut(V) -> String + 'static,
) where
    V: Clone,
{
    use_effect(move || {
        let value = value_ref.read();
        set_style(mounted, &make_style(value.clone()));
    });
}

fn set_style(mounted: UseMounted, style: &str) {
    if let Some(element) = &*mounted.signal.read() {
        let raw_elem = element.downcast::<web_sys::Element>().unwrap();

        raw_elem.set_attribute("style", style).unwrap();
    }
}
