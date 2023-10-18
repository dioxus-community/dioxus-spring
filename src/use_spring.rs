use crate::use_spring_signal;
use dioxus::prelude::{use_effect, Scope};
use dioxus_signals::Signal;
use interpolation::Lerp;
use std::time::Duration;

pub fn use_spring<T, V>(cx: Scope<T>, value: V, duration: Duration) -> Signal<V>
where
    V: PartialEq + Lerp<Scalar = f32> + Clone + 'static,
{
    let (spring_ref, signal) = use_spring_signal(cx, value.clone());

    use_effect(cx, &value, move |to| async move {
        spring_ref.animate(to, duration);
    });

    signal
}
