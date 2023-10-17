use std::rc::Rc;
use dioxus::prelude::{MountedData, Scope};
use dioxus_signals::Signal;
use interpolation::Lerp;

/// Mount an animated value to an element's style attribute.
pub fn use_animated<T, V>(
    cx: Scope<T>,
    element_ref: Signal<Option<Rc<MountedData>>>,
    value_ref: Signal<V>,
    mut make_style: impl FnMut(V) -> String + 'static,
) where
    V: Lerp<Scalar = f32> + PartialEq + Clone + 'static,
{
    dioxus_signals::use_effect(cx, move || {
        let value = value_ref.read();
        set_style(element_ref, &make_style(value.clone()));
    })
}

fn set_style(element_ref: Signal<Option<Rc<MountedData>>>, style: &str) {
    if let Some(element) = &*element_ref.read() {
        let raw_elem = element
            .get_raw_element()
            .unwrap()
            .downcast_ref::<web_sys::Element>()
            .unwrap();

        raw_elem.set_attribute("style", style).unwrap();
    }
}
