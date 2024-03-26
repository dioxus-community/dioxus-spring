use futures::Future;
use slotmap::{DefaultKey, SlotMap};
use std::{
    cell::RefCell,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};
use wasm_bindgen::{prelude::Closure, JsCast};

pub async fn request_animation_frame() {
    RequestFuture { key: None }.await
}

#[derive(Default)]
struct Control {
    wakers: SlotMap<DefaultKey, Waker>,
    pending: Option<Rc<Closure<dyn FnMut()>>>,
}

thread_local! {
    static CONTROL: RefCell<Option<Control>> = RefCell::new(None);
}

struct RequestFuture {
    key: Option<DefaultKey>,
}

impl Future for RequestFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        CONTROL
            .try_with(|cell| {
                if let Some(key) = self.key {
                    let maybe_controller = cell.borrow();
                    let controller = maybe_controller.as_ref().unwrap();
                    if controller.wakers.get(key).is_none() {
                        Poll::Ready(())
                    } else {
                        Poll::Pending
                    }
                } else {
                    let mut maybe_controller = cell.borrow_mut();
                    if maybe_controller.is_none() {
                        *maybe_controller = Some(Control::default());
                    }
                    let controller = maybe_controller.as_mut().unwrap();

                    let key = controller.wakers.insert(cx.waker().clone());
                    self.key = Some(key);

                    if controller.pending.is_none() {
                        let f: Closure<dyn FnMut()> = Closure::new(move || {
                            CONTROL
                                .try_with(|cell| {
                                    let mut maybe_controller = cell.borrow_mut();
                                    let controller = maybe_controller.as_mut().unwrap();

                                    for waker in controller.wakers.values() {
                                        waker.wake_by_ref();
                                    }
                                    controller.wakers.clear();
                                    controller.pending.take();
                                })
                                .unwrap();
                        });

                        web_sys::window()
                            .unwrap()
                            .request_animation_frame(f.as_ref().as_ref().unchecked_ref())
                            .unwrap();
                        controller.pending = Some(Rc::new(f));
                    }

                    Poll::Pending
                }
            })
            .unwrap()
    }
}
