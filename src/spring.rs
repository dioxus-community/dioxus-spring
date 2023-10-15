use crate::controller::request_animation_frame;
use futures::{stream, Stream};
use interpolation::Lerp;
use js_sys::Date;
use std::time::Duration;

pub fn spring<V>(from: V, to: V, duration: Duration) -> impl Stream<Item = V>
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let start = Date::now();

    stream::unfold(false, move |is_done| {
        let from = from.clone();
        let to = to.clone();
        async move {
            if is_done {
                return None;
            }

            request_animation_frame().await;

            let dt = Date::now() - start;
            let duration_ms = duration.as_secs_f64() * 1000.;
            if dt >= duration_ms {
                let v = interpolation::lerp(&from, &to, &1.);
                return Some((v, true));
            }

            let x = dt / duration_ms;
            let v = interpolation::lerp(&from, &to, &(x as f32));
            Some((v, false))
        }
    })
}
