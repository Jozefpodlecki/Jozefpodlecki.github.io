use yew::*;

use crate::models::Experience;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ExperienceProps {
    pub items: Vec<Experience>,
}

#[function_component(ExperienceSection)]
pub fn app(props: &ExperienceProps) -> Html {
   
    html! {
        <section class="py-5">
            <h1 class="font-[oswald] text-2xl font-bold text-gray-900 dark:text-gray-200">{"Experience"}</h1>
            {
                for props.items.iter().map(|exp| html! {
                    <div class="border-b border-gray-300 pb-4 mt-4">
                        <div class="flex justify-between items-center">
                            <div>
                                <h3 class="font-[roboto] font-semibold text-xl text-gray-800 dark:text-gray-300">
                                    { &exp.company }
                                </h3>
                                <p class="font-[roboto] text-gray-500 text-sm italic dark:text-gray-300">
                                    { format!("{} · {} · {}", exp.role, exp.employment_type, exp.work_mode) }
                                </p>
                            </div>
                            <p class="text-gray-500 text-sm dark:text-gray-300">{ &exp.period }</p>
                        </div>
                        <p class="font-[roboto] text-gray-500 text-sm dark:text-gray-300">{ &exp.location }</p>
                        <ul class="mt-2 list-none list-inside space-y-2 dark:text-gray-300">
                            {
                                for exp.responsibilities.iter().map(|resp| html! {
                                    <li class="font-[roboto] text-gray-700 text-sm dark:text-gray-300">{ resp }</li>
                                })
                            }
                        </ul>
                    </div>
                })
            }
        </section>
    }

}