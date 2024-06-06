use crate::use_spring_signal;
use dioxus::{hooks::use_reactive, prelude::use_memo};
use dioxus_signals::Signal;
use interpolation::Lerp;
use std::time::Duration;

/// Hook to create an animated signal from a reactive value and [`Duration`].
/// 
/// When `value` is changed, this signal will linearly interpolate from the current value to `value`.
pub fn use_spring<V>(value: V, duration: Duration) -> Signal<V>
where
    V: PartialEq + Lerp<Scalar = f32> + Clone + 'static,
{
    let (signal, spring_ref) = use_spring_signal(value.clone());

    use_memo(use_reactive((&value,), move |(to,)| {
        spring_ref.animate(to, duration);
    }));

    signal
}
