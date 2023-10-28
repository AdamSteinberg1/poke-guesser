use ::yew::prelude::*;
use itertools::Itertools;
use structmap::{value::Value, FromMap, GenericMap, ToMap};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::models::settings::Settings;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_settings_change: Callback<Settings>,
}

#[function_component]
pub fn SettingsMenu(Props { on_settings_change }: &Props) -> Html {
    let on_change = {
        let on_settings_change = on_settings_change.clone();
        Callback::from(move |event: Event| {
            let element: HtmlInputElement = event.target_unchecked_into();
            let siblings = element.parent_element().unwrap().children();
            let pairs = (0..siblings.length())
                .filter_map(|i| {
                    siblings
                        .item(i)
                        .and_then(|element| element.dyn_into::<HtmlInputElement>().ok())
                        .map(|input| (input.id(), Value::new(input.checked())))
                })
                .collect::<GenericMap>();
            on_settings_change.emit(Settings::from_genericmap(pairs));
        })
    };

    let menu_content: Html = Settings::to_genericmap(Settings::default())
        .into_iter()
        .map(|(id, _)| {
            let name = id_to_name(&id);
            html! {
                <>
                    <label for={id.clone()}>{name.clone()}</label>
                    <input type="checkbox" id={id.clone()} onchange={on_change.clone()}/>
                    <br/>
                </>
            }
        })
        .collect();

    html! {
        <div class="settings-menu">
            {menu_content}
        </div>
    }
}

fn id_to_name(id: &str) -> String {
    id.replace("_", " ")
        .split_whitespace()
        .map(|s| {
            let mut chars = s.chars();
            chars
                .next()
                .into_iter()
                .flat_map(|c| c.to_uppercase())
                .chain(chars)
                .collect::<String>()
        })
        .join(" ")
}
