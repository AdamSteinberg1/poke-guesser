use ::yew::prelude::*;
use rand::{thread_rng, Rng};
use yew_hooks::use_interval;

#[function_component]
pub fn Starburst() -> Html {
    let trigger = use_force_update();
    let fps = 10; //frames per second
    use_interval(
        move || {
            trigger.force_update();
        },
        1000 / fps,
    );

    let points = points(20.0, 50.0);
    let inner_points = points
        .iter()
        .cloned()
        .map(|(x, y)| (0.65 * x + 50.0, 0.65 * y + 50.0))
        .map(|(x, y)| format!("{},{} ", x, y))
        .collect::<String>();

    let outer_points = points
        .into_iter()
        .map(|(x, y)| (x + 50.0, y + 50.0))
        .map(|(x, y)| format!("{},{} ", x, y))
        .collect::<String>();

    html! {
        <svg version="1.1"
                viewBox="0 0 100 100"
                xmlns="http://www.w3.org/2000/svg">

            <filter id="blur">
                <@{"feGaussianBlur"} in="SourceGraphic" stdDeviation="0.4" />
            </filter>

            <polygon fill="#68C6E8" filter="url(#blur)" points={outer_points}/>
            <polygon fill="#f1fffa" filter="url(#blur)" points={inner_points}/>
        </svg>
    }
}

fn points(inner: f32, outer: f32) -> Vec<(f32, f32)> {
    let num_spikes = 1000;
    let delta_theta = std::f32::consts::TAU / num_spikes as f32;
    (0..num_spikes)
        .map(|i| {
            let theta = i as f32 * delta_theta;
            let rand_radius = thread_rng().gen_range(inner..outer);

            let x = rand_radius * theta.cos();
            let y = rand_radius * theta.sin();

            (x, y)
        })
        .collect()
}
