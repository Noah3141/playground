use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! { 
        <>
            <h1>{"hello there"}</h1> 
            <p class="mt-3">
                <Link>
                    {"Go to about"}
                </Link></p>
        </>
    }
}