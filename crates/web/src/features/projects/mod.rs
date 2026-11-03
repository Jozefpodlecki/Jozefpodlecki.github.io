mod artwork;
mod cards;
mod collection;
mod detail;
mod pagination;
pub mod state;

pub use collection::ProjectCollection;
pub use detail::ProjectPage;

use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <main data-component="projects-page" class="min-h-[65vh]">
            <header class="page-heading select-none">
                <h1 class="text-highlight">{"Projects"}</h1>
                <p class="my-3 text-[clamp(1.3rem,2.8vw,2rem)] text-subtle">{"A selection of ideas, experiments, and tools."}</p>
            </header>
            <ProjectCollection filterable={true} />
        </main>
    }
}
