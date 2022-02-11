use yew::prelude::*;
use yew::{classes, html};

#[function_component]
pub fn Spinner() -> Html {
    let g = if false { "lds-roller" } else { "" };

    html! {
      <div class={classes!(g)}>
            <div />
            <div />
            <div />
            <div />
            <div />
            <div />
            <div />
            <div />
        </div>
    }
}
