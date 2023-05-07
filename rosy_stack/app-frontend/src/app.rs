//? Contains the App first layer structure. Mainly routing.

use yew_router::prelude::*;
use yew::prelude::*;

// we are submerged one layer down already, even though the FILE SYSTEM shows app and main together, we are a mod in crate
use super::components::{ // Must say super/crate because app is a mod, and so its references need to be thought of as submerged
    pages::{
        home::*,
    },
    inc::{

    }
};


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,


    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },

        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}