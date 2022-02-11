use yew::prelude::*;

use crate::components::Spinner::*;

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <NavBar />
            <button {onclick}>{ "+1" } </button>
            <p>{ *counter }</p>
        </div>
    }
}

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <div>
            <Spinner />
        </div>
    }
}
