use dioxus::prelude::*;
use futures::{pin_mut, StreamExt};
use interpolation::Lerp;
use std::time::Duration;

mod controller;
pub use controller::request_animation_frame;

mod spring;
pub use spring::spring;

mod use_animated;
pub use use_animated::use_animated;

mod use_spring;
pub use use_spring::use_spring;

mod use_spring_signal;
pub use use_spring_signal::use_spring_signal;

mod use_spring_ref;
pub use use_spring_ref::{use_spring_ref, UseSpringRef};

pub fn use_on_spring<V>(from: V, to: V, duration: Duration, f: impl FnMut(V) + 'static)
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let mut cell = Some((from, to, f));
    use_future(move || {
        let (from, to, mut f) = cell.take().unwrap();
        async move {
            let spring = spring(from, to, duration);
            pin_mut!(spring);

            while let Some(val) = spring.next().await {
                f(val);
            }
        }
    });
}
