use ::yew::prelude::*;

#[function_component]
pub fn RepoLink() -> Html {
    let icon = Html::from_html_unchecked(include_str!("../../assets/github-mark.svg").into());
    html! {
        <a class="repo-link" href="https://github.com/AdamSteinberg1/poke-guesser" target="_blank">
            {icon}
        </a>
    }
}
