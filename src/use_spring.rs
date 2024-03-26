use crate::use_spring_signal;
use dioxus::{hooks::use_reactive, prelude::use_effect};
use dioxus_signals::Signal;
use interpolation::Lerp;
use std::time::Duration;

pub fn use_spring<V>(value: V, duration: Duration) -> Signal<V>
where
    V: PartialEq + Lerp<Scalar = f32> + Clone + 'static,
{
    let (mut spring_ref, signal) = use_spring_signal(value.clone());

    use_effect(use_reactive((&value,), move |(to,)| {
        spring_ref.animate(to, duration);
    }));

    signal
}
