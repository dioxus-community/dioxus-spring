use dioxus::prelude::*;
use futures::{pin_mut, StreamExt};
use interpolation::Lerp;
use std::{rc::Rc, time::Duration};

mod controller;
pub use controller::request_animation_frame;

mod spring;
pub use spring::spring;

pub fn use_spring<T, V>(
    cx: Scope<T>,
    from: V,
    to: V,
    duration: Duration,
    mut f: impl FnMut(V) + 'static,
) where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    cx.spawn(async move {
        let spring = spring(from, to, duration);
        pin_mut!(spring);

        while let Some(val) = spring.next().await {
            f(val);
        }
    });
}

pub fn use_spring_ref<T, V>(
    cx: Scope<T>,
    from: V,
    to: V,
    duration: Duration,
    mut make_style: impl FnMut(V) -> String + 'static,
) -> SpringRef
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let element_ref: UseRef<Option<Rc<MountedData>>> = use_ref(cx, || None).clone();

    let element_ref_clone = element_ref.clone();
    use_spring(cx, from, to, duration, move |val| {
        if let Some(element) = &*element_ref_clone.read() {
            let raw_elem = element
                .get_raw_element()
                .unwrap()
                .downcast_ref::<web_sys::Element>()
                .unwrap();

            raw_elem.set_attribute("style", &make_style(val)).unwrap();
        }
    });

    SpringRef { element_ref }
}

pub struct SpringRef {
    element_ref: UseRef<Option<Rc<MountedData>>>,
}

impl SpringRef {
    pub fn mount(&self, data: Rc<MountedData>) {
        self.element_ref.set(Some(data));
    }
}
