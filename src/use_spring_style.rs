use crate::{use_spring_ref, UseSpringRef};
use dioxus::prelude::*;
use interpolation::Lerp;
use std::{rc::Rc, time::Duration};

pub fn use_spring_style<T, V>(
    cx: Scope<T>,
    from: V,
    to: V,
    duration: Duration,
    mut make_style: impl FnMut(V) -> String + 'static,
) -> &UseSpringStyle
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let element_ref: UseRef<Option<Rc<MountedData>>> = use_ref(cx, || None).clone();

    let element_ref_clone = element_ref.clone();
    let spring_ref = use_spring_ref(cx, from, to, duration, move |val| {
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

pub struct UseSpringStyle {
    element_ref: UseRef<Option<Rc<MountedData>>>,
    spring_ref: UseSpringRef,
}

impl UseSpringStyle {
    pub fn mount(&self, data: Rc<MountedData>) {
        self.element_ref.set(Some(data));
    }

    pub fn start(&self) {
        self.spring_ref.start();
    }
}
