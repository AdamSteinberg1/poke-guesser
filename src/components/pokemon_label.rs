use ::yew::prelude::*;

use crate::models::pokemon::PokemonType;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or(true)]
    pub is_revealed: bool,
    pub name: AttrValue,
    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,
}

#[function_component]
pub fn PokemonLabel(
    Props {
        is_revealed,
        name,
        primary_type,
        secondary_type,
    }: &Props,
) -> Html {
    let visibility = if *is_revealed { "visible" } else { "hidden" };
    let name = html! {
        <svg class="pokemon-name" xmlns="http://www.w3.org/2000/svg">
            <text x="50%" y="50%" visibility={visibility}>
                {name}
            </text>
        </svg>
    };
    let primary_type = type_label(primary_type);
    let secondary_type = secondary_type.as_ref().map(type_label);
    html! {
        <>
            {name}
            if *is_revealed {
                <p class="pokemon-type">
                    {primary_type}
                    {secondary_type}
                </p>
            }
        </>
    }
}

fn type_label(t: &PokemonType) -> Html {
    html! {
        <span
            style={format!("background-color:{}", t.color())}>
                {t.to_string()}
        </span>
    }
}
