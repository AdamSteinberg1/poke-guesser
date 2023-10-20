use ::yew::prelude::*;

#[function_component]
pub fn Title() -> Html {
    html! {
        <>
            <img class = "upper-title" src="assets/whos.png"/>
            <img class = "upper-title" src="assets/that.png"/>
            <br/>
            <img class = "lower-title" src="assets/pokemon_logo.svg"/>
            <img class = "lower-title" src="assets/question_mark.png"/>
        </>
    }
}
