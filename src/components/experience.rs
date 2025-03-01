use yew::*;

use crate::models::Experience;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub items: Vec<Experience>,
}

#[function_component(ExperienceSection)]
pub fn app(props: &Props) -> Html {
    let show_all = use_state(|| false);

    let visible_items = if *show_all {
        props.items.clone()
    } else {
        props.items.iter().take(2).cloned().collect::<Vec<_>>()
    };

    let on_show_more = {
        let show_all = show_all.clone();
        Callback::from(move |_| show_all.set(true))
    };

    html! {
        <section class="py-5">
            <h1 class="font-[oswald] text-2xl font-bold text-gray-900 dark:text-gray-200">{"Experience"}</h1>
            {
                for visible_items.iter().map(|exp| html! {
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
            if !*show_all {
               <button
                    class="bg-white dark:bg-black mt-2 hover:bg-gray-100 text-gray-800 dark:text-gray-200 font-semibold py-2 px-4 border border-gray-400 rounded shadow cursor-pointer"
                    onclick={on_show_more}>
                    {"More"}
                </button>
            }
        </section>
    }
}
