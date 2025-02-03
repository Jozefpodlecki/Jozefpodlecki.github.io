use yew::*;

use crate::{icons::LinkIcon, models::Project};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub items: Vec<Project>,
}

#[function_component(ProjectsSection)]
pub fn app(props: &Props) -> Html {
   
    html! {
        <section class="py-5">
            <h1 class="font-[oswald] text-2xl font-bold text-gray-900 dark:text-gray-200">{"Projects"}</h1>
            <ul class="mt-6 space-y-6">
                {
                    for props.items.iter().map(|project| html! {
                        <li class="flex flex-col md:flex-row md:items-center p-4 bg-white dark:bg-gray-800 shadow-sm transition">
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
                            </div>
                        </li>
                    })
                }
            </ul>
        </section>
    }

}