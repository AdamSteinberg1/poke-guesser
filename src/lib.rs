use crate::{
    components::{
        guesser::Guesser, repo_link::RepoLink, settings_menu::SettingsMenu, title::Title,
    },
    models::settings::Settings,
};
use yew::prelude::*;

mod components;
mod models;
mod util;

#[function_component]
pub fn App() -> Html {
    let settings = use_state(|| Settings::default());

    let on_settings_change = {
        let settings = settings.clone();
        Callback::from(move |new_settings: Settings| {
            settings.set(new_settings);
        })
    };

    let fallback = html! {<img class = "loading" src="assets/question_mark.png"/>};
    html! {
        <>
            <RepoLink/>
            <SettingsMenu on_settings_change={on_settings_change}/>
            <Title/>
            <Suspense {fallback}>
                <Guesser settings={(*settings).clone()}/>
            </Suspense>
        </>
    }
}
