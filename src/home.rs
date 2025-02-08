use yew::*;

use crate::{app_state::AppState, experience::ExperienceSection, icons::*, layout::Layout, projects_section::ProjectsSection};

#[function_component(Home)]
pub fn home() -> Html {
    let app_state = use_context::<AppState>().unwrap();
    let summary = app_state.summary;

    html! {
    <div class="">
        <Layout>
            <section class="py-5 flex items-center">
                <div>
                    <span class="font-[oswald] block text-3xl font-bold text-gray-900 dark:text-gray-200">{summary.full_name.clone()}</span>
                    <span class="font-[oswald] block text-xl text-gray-700 dark:text-gray-200">{summary.role.clone()}</span>

                    <div class="flex justify-start gap-1 mt-2">
                        <a class="dark:text-white" href={summary.social.linkedin.clone()}><Linkedin/></a>
                        <a class="dark:text-white" href={summary.social.github.clone()}><Github/></a>
                        <a class="dark:text-white" href={summary.social.codepen.clone()}><Codepen/></a>
                        <a class="dark:text-white" href={summary.social.duolingo.clone()}><Duolingo/></a>
                    </div>
                </div>
                <img class="w-[150px] ml-auto hidden md:block" src="public/avatar.jpg"/>
            </section>

            <ProjectsSection items={summary.projects.clone()} />

            <ExperienceSection items={summary.experience.clone()} />

            <footer class="font-[roboto] py-2 mt-auto text-center text-sm text-gray-500 dark:text-gray-300">
                <span>{"© Jozef Podlecki 2025"}</span>
            </footer>
        </Layout>
    </div>
    }
}