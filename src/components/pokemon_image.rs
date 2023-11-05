use crate::components::starburst::Starburst;
use ::yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub silhouette: bool,
    pub image: String,
}

#[function_component]
pub fn PokemonImage(Props { silhouette, image }: &Props) -> Html {
    let filter = format!("url(#{})", if *silhouette { "hidden" } else { "normal" });
    html! {
        <svg xmlns="http://www.w3.org/2000/svg"
            class="pokemon"
            viewBox="0 0 100 100">
            <defs>
                <filter id="hidden" color-interpolation-filters="sRGB">
                    <@{"feColorMatrix"} type="matrix"
                        values="0 0 0 0 0.04313
                            0 0 0 0 0.40784
                            0 0 0 0 0.63137
                            0 0 0 1 0"/>
                    <@{"feDropShadow"} dx="-0.5" dy="0.5" stdDeviation="0.2" />
                </filter>
                <filter id="normal" color-interpolation-filters="sRGB">
                    <@{"feDropShadow"} dx="-0.5" dy="0.5" stdDeviation="0.2" />
                </filter>
            </defs>
            <Starburst/>
            <image href={image.clone()}
                width="60"
                height="60"
                x="20"
                y="20"
                filter={filter}/>
        </svg>
    }
}
