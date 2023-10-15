use crate::spring;
use dioxus::prelude::*;
use futures::{pin_mut, StreamExt};
use interpolation::Lerp;
use std::time::Duration;

pub fn use_spring_ref<T, V>(cx: Scope<T>, mut f: impl FnMut(V) + 'static) -> &UseSpringRef<V>
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let (tx, rx) = cx.use_hook(|| async_channel::unbounded());
    to_owned![tx, rx];

    cx.spawn(async move {
        while let Some((from, to, duration)) = rx.next().await {
            let spring = spring(from, to, duration);
            pin_mut!(spring);

            while let Some(val) = spring.next().await {
                f(val);
            }
        }
    });

    cx.bump().alloc(UseSpringRef { tx })
}

#[derive(Clone)]
pub struct UseSpringRef<V> {
    tx: async_channel::Sender<(V, V, Duration)>,
}

impl<V> UseSpringRef<V> {
    pub fn start(&self, from: V, to: V, duration: Duration) {
        self.tx.send_blocking((from, to, duration)).unwrap();
    }
}
