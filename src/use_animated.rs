use dioxus::prelude::Scope;
use dioxus_signals::Signal;
use dioxus_use_mounted::UseMounted;

pub fn use_animated<T, V>(
    cx: Scope<T>,
    mounted: UseMounted,
    value_ref: Signal<V>,
    mut make_style: impl FnMut(V) -> String + 'static,
) where
    V: Clone,
{
    dioxus_signals::use_effect(cx, move || {
        let value = value_ref.read();
        set_style(mounted, &make_style(value.clone()));
    })
}

fn set_style(mounted: UseMounted, style: &str) {
    if let Some(element) = &*mounted.signal.read() {
        let raw_elem = element
            .get_raw_element()
            .unwrap()
            .downcast_ref::<web_sys::Element>()
            .unwrap();

        raw_elem.set_attribute("style", style).unwrap();
    }
}
