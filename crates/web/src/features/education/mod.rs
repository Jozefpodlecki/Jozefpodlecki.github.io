use crate::content::Education;
use yew::prelude::*;
mod entry;

use self::entry::EducationEntry;

#[derive(Properties, PartialEq)]
pub struct EducationSectionProps {
    pub items: Vec<Education>,
}

#[function_component(EducationSection)]
pub fn education_section(props: &EducationSectionProps) -> Html {
    if props.items.is_empty() {
        return Html::default();
    }

    html! {
        <section
            data-component="education-section"
            aria-labelledby="education-heading"
            class="section relative py-10 sm:py-14"
        >
            <h2
                id="education-heading"
                class="!m-0 select-none text-2xl font-semibold tracking-tight sm:text-3xl"
            >
                {"Education"}
            </h2>
            <div class="mt-5 flex flex-col gap-4">
                {for props.items.iter().cloned().map(|item| html! {
                    <EducationEntry item={item} />
                })}
            </div>
        </section>
    }
}
