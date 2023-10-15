use controller::request_animation_frame;
use dioxus::prelude::*;
use futures::StreamExt;
use futures_channel::mpsc;
use interpolation::Lerp;
use js_sys::Date;
use std::{fmt, rc::Rc, time::Duration};
use wasm_bindgen::{prelude::Closure, JsCast};

mod controller;

pub fn use_spring<T, V>(
    cx: Scope<T>,
    from: V,
    to: V,
    duration: Duration,
    mut make_style: impl FnMut(V) -> String + 'static,
) -> SpringRef
where
    V: Lerp<Scalar = f32> + fmt::Display + 'static,
{
    let element_ref: UseRef<Option<Rc<MountedData>>> = use_ref(cx, || None).clone();
    let start = *cx.use_hook(|| Date::now());

    let element_ref_clone = element_ref.clone();
    cx.spawn(async move {
        loop {
            let dt = Date::now() - start;
            if dt >= duration.as_secs_f64() * 1000. {
                break;
            }

            if let Some(element) = &*element_ref_clone.read() {
                let raw_elem = element
                    .get_raw_element()
                    .unwrap()
                    .downcast_ref::<web_sys::Element>()
                    .unwrap();

                let x = dt / (duration.as_secs_f64() * 1000.);
                let v = interpolation::lerp(&from, &to, &(x as f32));
                raw_elem.set_attribute("style", &make_style(v)).unwrap();
            }

            request_animation_frame().await;
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
