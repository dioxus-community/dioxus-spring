use crate::spring;
use dioxus::prelude::*;
use futures::StreamExt;
use interpolation::Lerp;
use std::{task::Poll, time::Duration};

pub fn use_on_spring<T, V>(
    cx: Scope<T>,
    from: V,
    mut f: impl FnMut(V) + 'static,
) -> &UseSpringRef<V>
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let (tx, rx) = cx.use_hook(async_channel::unbounded);
    to_owned![ tx, rx ];

    let mut current = from;
    let mut cell = None;
    use_future(cx, (), move |_| {
        futures::future::poll_fn(move |cx| {
            while let Poll::Ready(Some((to, duration_cell))) = rx.poll_next_unpin(cx) {
                if let Some(duration) = duration_cell {
                    let spring = spring(current.clone(), to, duration);
                    cell = Some(Box::pin(spring));
                } else {
                    current = to.clone();
                    cell = None;
                    f(to);
                }
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

pub struct UseSpringRef<V> {
    tx: async_channel::Sender<(V, Option<Duration>)>,
}

impl<V> UseSpringRef<V> {
    pub fn set(&self, to: V) {
        self.tx.send_blocking((to, None)).unwrap();
    }

    pub fn animate(&self, to: V, duration: Duration) {
        self.tx.send_blocking((to, Some(duration))).unwrap();
    }
}

impl<V> Clone for UseSpringRef<V> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
