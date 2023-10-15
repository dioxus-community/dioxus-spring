use dioxus::prelude::*;
use futures::StreamExt;
use futures_channel::mpsc;
use interpolation::Lerp;
use js_sys::Date;
use std::{fmt, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};

pub fn use_spring<T, V>(cx: Scope<T>, from: V, to: V, mut make_style: impl FnMut(V) -> String + 'static) -> SpringRef
where
    V: Lerp<Scalar = f32> + fmt::Display + 'static,
{
    let element_ref: UseRef<Option<Rc<MountedData>>> = use_ref(cx, || None).clone();
    let start = *cx.use_hook(|| Date::now());

    let element_ref_clone = element_ref.clone();
    cx.spawn(async move {
        for _ in 0..100 {
            if let Some(element) = &*element_ref_clone.read() {
                let raw_elem = element
                    .get_raw_element()
                    .unwrap()
                    .downcast_ref::<web_sys::Element>()
                    .unwrap();

                let dt = Date::now() - start;
                let v = interpolation::lerp(&from, &to, &((dt / 1000.) as f32));

                raw_elem
                    .set_attribute("style", &make_style(v))
                    .unwrap();
            }

            let (tx, mut rx) = mpsc::unbounded();
            let f: Closure<dyn FnMut()> = Closure::new(move || {
                tx.unbounded_send(()).unwrap();
            });
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.as_ref().unchecked_ref())
                .unwrap();
            rx.next().await;
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
