use yew::*;

use crate::{app_state::AppState, breadcrumbs::Breadcrumbs, icons::LinkIcon, layout::Layout};

#[function_component(Projects)]
pub fn projects() -> Html {
    let app_state = use_context::<AppState>().unwrap();
    let projects = app_state.summary.projects;

    html! {
        <div class="">
            <Layout>
                <Breadcrumbs/>

                <h1 class="font-[oswald] text-2xl font-bold text-gray-900 dark:text-gray-200">{"Projects"}</h1>
                <ul class="mt-2">
                    {
                        for projects.iter().cloned().map(|project| html! {
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
                                </div>
                            </li>
                        })
                    }
                </ul>

                <footer class="font-[roboto] py-2 mt-auto text-center text-sm text-gray-500 dark:text-gray-300">
                    <span>{"© Jozef Podlecki 2025"}</span>
                </footer>
            </Layout>
        </div>
    }

}