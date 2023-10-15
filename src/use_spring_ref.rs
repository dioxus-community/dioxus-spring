use crate::spring;
use dioxus::prelude::*;
use futures::{pin_mut, StreamExt};
use interpolation::Lerp;
use std::time::Duration;

pub fn use_spring_ref<T, V>(
    cx: Scope<T>,
    from: V,
    to: V,
    duration: Duration,
    mut f: impl FnMut(V) + 'static,
) -> &UseSpringRef
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let (tx, rx) = cx.use_hook(|| async_channel::unbounded());
    to_owned![tx, rx];

    cx.spawn(async move {
        while rx.next().await.is_some() {
            let spring = spring(from.clone(), to.clone(), duration);
            pin_mut!(spring);

            while let Some(val) = spring.next().await {
                f(val);
            }
        }
    });

    cx.bump().alloc(UseSpringRef { tx })
}

#[derive(Clone)]
pub struct UseSpringRef {
    tx: async_channel::Sender<()>,
}

impl UseSpringRef {
    pub fn start(&self) {
        self.tx.send_blocking(()).unwrap();
    }
}
