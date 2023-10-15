use dioxus::prelude::RefCell;
use futures::StreamExt;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast};

struct Controller {
    tx: async_channel::Sender<()>,
    rx: async_channel::Receiver<()>,
    pending: Option<Rc<Closure<dyn FnMut()>>>,
}

thread_local! {
    static CONTROLLER: RefCell<Option<Controller>> = RefCell::new(None);
}

pub async fn request_animation_frame() {
    let (request_cell, mut rx) = CONTROLLER
        .try_with(|cell| {
            let mut cx = cell.borrow_mut();
            let controller = if let Some(controller) = &mut *cx {
                controller
            } else {
                let (tx, rx) = async_channel::unbounded();
                *cx = Some(Controller {
                    tx,
                    rx,
                    pending: None,
                });
                cx.as_mut().unwrap()
            };

            let request_cell = if controller.pending.is_none() {
                let tx = controller.tx.clone();
                let f: Closure<dyn FnMut()> = Closure::new(move || {
                    tx.send_blocking(()).unwrap();

                    CONTROLLER
                        .try_with(|cell| {
                            let mut cx = cell.borrow_mut();
                            cx.as_mut().unwrap().pending.take();
                        })
                        .unwrap();
                });

                controller.pending = Some(Rc::new(f));
                controller.pending.clone()
            } else {
                None
            };
            (request_cell, controller.rx.clone())
        })
        .unwrap();

    if let Some(f) = request_cell {
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().as_ref().unchecked_ref())
            .unwrap();
    }

    rx.next().await;
}
