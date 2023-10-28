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
    let collapsed = use_state(|| true);

    let on_change = {
        let on_settings_change = on_settings_change.clone();
        Callback::from(move |event: Event| {
            let element: HtmlInputElement = event.target_unchecked_into();
            let siblings = element.parent_element().unwrap().children();
            let map: GenericMap = (0..)
                .map_while(|i| siblings.item(i))
                .filter_map(|elem| {
                    elem.dyn_into::<HtmlInputElement>()
                        .map(|input| (input.id(), Value::new(input.checked())))
                        .ok()
                })
                .collect();
            on_settings_change.emit(Settings::from_genericmap(map));
        })
    };

    let menu_content: Html = Settings::to_genericmap(Settings::default())
        .into_iter()
        .map(|(id, _)| {
            let name = id_to_name(&id);
            html! {
                <>
                    <br/>
                    <input
                        type="checkbox" id={id.clone()}
                        onchange={on_change.clone()}
                        style={collapsed.then_some("max-height:0; margin:0; opacity:0")}/>
                    <label
                        for={id}
                        style={collapsed.then_some("opacity:0")}>
                        {name}
                    </label>
                </>
            }
        })
        .collect();

    let on_collapse = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| {
            collapsed.set(true);
        })
    };
    let on_expand = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| {
            collapsed.set(false);
        })
    };

    let gear = Html::from_html_unchecked(include_str!("../../assets/settings.svg").into());
    let x = Html::from_html_unchecked(include_str!("../../assets/x.svg").into());
    html! {
        <div class="settings-menu" style={collapsed.then_some("font-size:0; border-radius:100%;")}>
            if *collapsed {
                <span onclick={on_expand}>{gear}</span>
            } else {
                <span onclick={on_collapse}>{x}</span>
            }
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
