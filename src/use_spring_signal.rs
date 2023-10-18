use crate::use_spring_ref::{use_spring_ref, Message};
use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use interpolation::Lerp;
use std::time::Duration;

pub fn use_spring_signal<T, V>(cx: Scope<T>, from: V) -> (UseSpringSignal<V>, Signal<V>)
where
    V: PartialEq + Lerp<Scalar = f32> + Clone + 'static,
{
    let from_clone = from.clone();
    let output = use_signal(cx, move || from_clone);

    let spring_ref = use_spring_ref(cx, from, move |value| output.set(value));
    to_owned![spring_ref];

    let signal: Signal<Option<Message<V>>> = use_signal(cx, || None);

    dioxus_signals::use_effect(cx, move || {
        if let Some(msg) = &*signal.read() {
            match msg {
                Message::Set(to, duration_cell) => {
                    if let Some(duration) = duration_cell {
                        spring_ref.animate(to.clone(), *duration);
                    } else {
                        spring_ref.set(to.clone());
                    }
                }
                Message::Queue(to, duration) => spring_ref.queue(to.clone(), *duration),
            }
        }
    });

    (UseSpringSignal { signal }, output)
}

pub struct UseSpringSignal<V: 'static> {
    signal: Signal<Option<Message<V>>>,
}

impl<V> UseSpringSignal<V> {
    pub fn set(&self, to: V) {
        self.signal.set(Some(Message::Set(to, None)))
    }

    pub fn animate(&self, to: V, duration: Duration) {
        self.signal.set(Some(Message::Set(to, Some(duration))))
    }

    pub fn queue(&self, to: V, duration: Duration) {
        self.signal.set(Some(Message::Queue(to, duration)))
    }
}

impl<V> Clone for UseSpringSignal<V> {
    fn clone(&self) -> Self {
        Self {
            signal: self.signal.clone(),
        }
    }
}

impl<V> Copy for UseSpringSignal<V> {}
