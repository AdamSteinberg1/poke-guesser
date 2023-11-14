use ::yew::prelude::*;

#[function_component]
pub fn RepoLink() -> Html {
    html! {
        <a class="repo-link" href="https://github.com/AdamSteinberg1/poke-guesser" target="_blank">
            <img src="assets/github-mark.svg"/>
        </a>
    }
}
