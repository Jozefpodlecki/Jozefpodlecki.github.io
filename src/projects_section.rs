use yew::*;
use yew_router::prelude::Link;

use crate::{icons::{CodeIcon, LinkIcon}, models::Project, route::Route};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub items: Vec<Project>,
}

#[function_component(ProjectsSection)]
pub fn projects_section(props: &Props) -> Html {
   
    html! {
        <section class="py-5">
            <h1 class="font-[oswald] text-2xl font-bold text-gray-900 dark:text-gray-200">{"Projects"}</h1>
            <ul class="mt-2">
                {
                    for props.items.iter().take(2).cloned().map(|project| html! {
                        <li class="flex flex-col md:flex-row md:items-center p-2 bg-white dark:bg-gray-800 shadow-sm transition">
                            <div class="flex-1">
                                <a 
                                    href={project.link.clone()} 
                                    target="_blank" 
                                    class="font-[roboto] dark:text-gray-200 text-lg font-semibold hover:underline">
                                    { &project.name } <LinkIcon class="inline" />
                                </a>
                                <p class="font-[roboto] text-gray-700 dark:text-gray-400 mt-1 text-sm">
                                    { &project.description }
                                </p>
                                <a 
                                    href={project.github.clone()} 
                                    target="_blank" 
                                    class="font-[roboto] dark:text-gray-200 text-lg">
                                    <CodeIcon class="inline" />
                                </a>
                            </div>
                        </li>
                    })
                }
            </ul>
            // <Link<Route> to={Route::Projects} classes="inline-block bg-white dark:bg-black mt-2 hover:bg-gray-100 text-gray-800 dark:text-gray-200 font-semibold py-2 px-4 border border-gray-400 rounded shadow cursor-pointer">
            //     { "More" }
            // </Link<Route>>
        </section>
    }

}