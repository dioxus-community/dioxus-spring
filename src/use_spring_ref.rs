use crate::spring;
use dioxus::prelude::*;
use futures::StreamExt;
use interpolation::Lerp;
use std::{collections::VecDeque, task::Poll, time::Duration};

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
    let mut spring_cell = None;
    let mut stack = VecDeque::new();
    use_future(cx, (), move |_| {
        futures::future::poll_fn(move |cx| {
            while let Poll::Ready(Some(msg)) = rx.poll_next_unpin(cx) {
                match msg {
                    Message::Set(to, duration_cell) => {
                        if let Some(duration) = duration_cell {
                            let spring = spring(current.clone(), to, duration);
                            spring_cell = Some(Box::pin(spring));
                            stack.clear();
                        } else {
                            current = to.clone();
                            spring_cell.take();
                            stack.clear();
                            f(to);
                        }
                    }
                    Message::Queue(to, duration) => {
                        stack.push_back((to, duration));
                    }
                }
            }

            if spring_cell.is_none() {
                if let Some((to, duration)) = stack.pop_front() {
                    let spring = crate::spring(current.clone(), to, duration);
                    spring_cell = Some(Box::pin(spring));
                } else {
                    spring_cell = None;
                }
            }

            while let Some(spring) = spring_cell.as_mut() {
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
                    if let Some((to, duration)) = stack.pop_front() {
                        let spring = crate::spring(current.clone(), to, duration);
                        spring_cell = Some(Box::pin(spring));
                    } else {
                        spring_cell = None;
                    }
                } else {
                    break;
                }
            }

            Poll::<()>::Pending
        })
    });

    cx.bump().alloc(UseSpringRef { tx })
}

pub(crate) enum Message<V> {
    Set(V, Option<Duration>),
    Queue(V, Duration),
}

pub struct UseSpringRef<V> {
    tx: async_channel::Sender<Message<V>>,
}

impl<V> UseSpringRef<V> {
    pub fn set(&self, to: V) {
        self.tx.send_blocking(Message::Set(to, None)).unwrap();
    }

    pub fn animate(&self, to: V, duration: Duration) {
        self.tx
            .send_blocking(Message::Set(to, Some(duration)))
            .unwrap();
    }

    pub fn queue(&self, to: V, duration: Duration) {
        self.tx.send_blocking(Message::Queue(to, duration)).unwrap();
    }
}

impl<V> Clone for UseSpringRef<V> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
