use ::yew::prelude::*;
use itertools::EitherOrBoth;
use itertools::Itertools;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub is_revealed: bool,
    pub on_reveal: Callback<MouseEvent>,
    pub on_new_pokemon: Callback<MouseEvent>,
    pub name: AttrValue,
}

#[function_component]
pub fn TextInput(
    Props {
        name,
        is_revealed,
        on_reveal,
        on_new_pokemon,
    }: &Props,
) -> Html {
    let on_submit = {
        let name = name.clone();
        let on_reveal = on_reveal.clone();
        Callback::from(move |event: MouseEvent| {
            let target: HtmlInputElement = event.target_unchecked_into();
            let text_input: HtmlInputElement =
                target.previous_element_sibling().unwrap().unchecked_into();
            let guess = text_input.value();
            if check_guess(&guess, &name) {
                on_reveal.emit(event);
            } else {
                text_input
                    .parent_element()
                    .map(|parent| parent.class_list().add_1("incorrect-guess"));
            }
        })
    };

    let onkeydown = Callback::from(|event: KeyboardEvent| {
        if event.key() == "Enter" {
            let target: HtmlInputElement = event.target_unchecked_into();
            let target = if target.id() == "submit" {
                target
            } else {
                target.next_element_sibling().unwrap().unchecked_into()
            };
            target.click();
        }
    });

    let onanimationend = Callback::from(move |event: AnimationEvent| {
        let target: HtmlElement = event.target_unchecked_into();
        let _ = target.class_list().remove_1("incorrect-guess");
    });

    html! {
        <>
            if *is_revealed {
                <button type="button" onclick={on_new_pokemon}>{"Next"}</button>
            } else {
                <div class="text-guess" {onanimationend}>
                    <input type="text" id="guess" onkeydown={onkeydown.clone()}/>
                    <input type="button" id="submit" value="🡆" onclick={on_submit} {onkeydown}/>
                </div>
                <button type="button" onclick={on_reveal.clone()}>{"Reveal"}</button>
            }
        </>
    }
}

fn check_guess(guess: &str, actual: &str) -> bool {
    let [guess, actual] = [guess, actual].map(|s| s.chars().filter(|c| c.is_ascii_alphanumeric()));

    guess.zip_longest(actual).all(|pair| match pair {
        EitherOrBoth::Both(guess, actual) => guess.eq_ignore_ascii_case(&actual),
        _ => false,
    })
}
