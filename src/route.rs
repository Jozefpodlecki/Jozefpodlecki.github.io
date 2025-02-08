use yew::*;
use yew_router::Routable;

use crate::{home::Home, not_found::NotFound, projects::Projects};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Projects => html! { <Projects/> },
        Route::NotFound => html! { <NotFound/> },
    }
}