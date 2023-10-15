use crate::spring;
use core::fmt;
use dioxus::prelude::*;
use futures::{pin_mut, StreamExt};
use interpolation::Lerp;
use std::time::Duration;

pub fn use_spring_ref<T, V>(
    cx: Scope<T>,
    from: V,
    mut f: impl FnMut(V) + 'static,
) -> &UseSpringRef<V>
where
    V: Lerp<Scalar = f32> + fmt::Debug + Clone + 'static,
{
    let (tx, rx) = cx.use_hook(|| async_channel::unbounded());
    to_owned![tx, rx];

    use_future(cx, (), move |_| async move {
        let mut current = from;

        while let Some((to, duration)) = rx.next().await {
            let spring = spring(current.clone(), to, duration);
            pin_mut!(spring);

            while let Some(val) = spring.next().await {
                current = val.clone();
                f(val);
            }
        }
    });

    cx.bump().alloc(UseSpringRef { tx })
}

#[derive(Clone)]
pub struct UseSpringRef<V> {
    tx: async_channel::Sender<(V, Duration)>,
}

impl<V> UseSpringRef<V> {
    pub fn start(&self, to: V, duration: Duration) {
        self.tx.send_blocking((to, duration)).unwrap();
    }
}
