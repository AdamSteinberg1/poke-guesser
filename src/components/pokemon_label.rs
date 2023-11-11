use ::yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub is_revealed: bool,
    pub name: AttrValue,
    pub id: u16,
}

#[function_component]
pub fn PokemonLabel(
    Props {
        is_revealed,
        id,
        name,
    }: &Props,
) -> Html {
    let visibility = if *is_revealed { "visible" } else { "hidden" };
    html! {
        <svg class="pokemon-name" xmlns="http://www.w3.org/2000/svg">
            <text x="50%" y="50%" visibility={visibility}>
                {format!("{}: {}", id, name)}
            </text>
        </svg>
    }
}
