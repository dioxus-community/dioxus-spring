use crate::spring;

use dioxus::prelude::*;
use futures::{pin_mut, StreamExt};
use interpolation::Lerp;
use std::{task::Poll, time::Duration};

pub fn use_spring_ref<T, V>(
    cx: Scope<T>,
    from: V,
    mut f: impl FnMut(V) + 'static,
) -> &UseSpringRef<V>
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let (tx, rx) = cx.use_hook(async_channel::unbounded);
    to_owned![tx, rx];

    let mut current = from;
    let mut cell = None;
    use_future(cx, (), move |_| {
        futures::future::poll_fn(move |cx| {
            if let Poll::Ready(Some((to, duration))) = rx.poll_next_unpin(cx) {
                let spring = spring(current.clone(), to, duration);
                cell = Some(Box::pin(spring));
            }

            if let Some(spring) = cell.as_mut() {
                let mut is_done = false;
                while let Poll::Ready(item) = spring.poll_next_unpin(cx) {
                    if let Some(val) = item {
                        current = val.clone();
                        f(val);
                    } else {
                        is_done = true;
                        break;
                    }
                }
                if is_done {
                    cell = None;
                }
            }

            Poll::<()>::Pending
        })
    });

    cx.bump().alloc(UseSpringRef { tx })
}

#[derive(Clone)]
pub struct UseSpringRef<V> {
    tx: async_channel::Sender<(V, Duration)>,
}

impl<V> UseSpringRef<V> {
    pub fn transition_to(&self, to: V, duration: Duration) {
        self.tx.send_blocking((to, duration)).unwrap();
    }
}
