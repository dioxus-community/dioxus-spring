use crate::{use_spring_ref, UseSpringRef};
use dioxus::prelude::*;
use interpolation::Lerp;
use std::{fmt, rc::Rc, time::Duration};

pub fn use_spring_style<T, V>(
    cx: Scope<T>,
    from: V,
    mut make_style: impl FnMut(V) -> String + 'static,
) -> &UseSpringStyle<V>
where
    V: Lerp<Scalar = f32> + fmt::Debug + Clone + 'static,
{
    let element_ref: UseRef<Option<Rc<MountedData>>> = use_ref(cx, || None).clone();

    let element_ref_clone = element_ref.clone();
    let from_clone = from.clone();
    use_effect(cx, &element_ref.read().is_some(), |_| {
        if let Some(element) = &*element_ref_clone.read() {
            let raw_elem = element
                .get_raw_element()
                .unwrap()
                .downcast_ref::<web_sys::Element>()
                .unwrap();

            raw_elem
                .set_attribute("style", &make_style(from_clone))
                .unwrap();
        }

        async {}
    });

    let element_ref_clone = element_ref.clone();
    let spring_ref = use_spring_ref(cx, from, move |val| {
        if let Some(element) = &*element_ref_clone.read() {
            let raw_elem = element
                .get_raw_element()
                .unwrap()
                .downcast_ref::<web_sys::Element>()
                .unwrap();

            raw_elem.set_attribute("style", &make_style(val)).unwrap();
        }
    });

    cx.bump().alloc(UseSpringStyle {
        element_ref,
        spring_ref: spring_ref.clone(),
    })
}

pub struct UseSpringStyle<V> {
    element_ref: UseRef<Option<Rc<MountedData>>>,
    spring_ref: UseSpringRef<V>,
}

impl<V> UseSpringStyle<V> {
    pub fn mount(&self, data: Rc<MountedData>) {
        self.element_ref.set(Some(data));
    }

    pub fn transition_to(&self, to: V, duration: Duration) {
        self.spring_ref.transition_to(to, duration);
    }
}
