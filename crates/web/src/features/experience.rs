use yew::prelude::*;
use yew_router::prelude::*;
use crate::content::Experience;
use crate::routes::Route;
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq)]
pub struct ExperienceSectionProps {
    pub items: Vec<Experience>,
    #[prop_or_default]
    pub detailed: bool,
}

#[derive(Properties, PartialEq)]
struct ExperienceEntryProps {
    item: Experience,
    detailed: bool,
    index: usize,
}

#[function_component(ExperienceEntry)]
fn experience_entry(props: &ExperienceEntryProps) -> Html {
    let item = &props.item;
    html! {
        <article
            data-component="experience-entry"
            data-experience-index={props.index.to_string()}
            class="group relative grid min-w-0 select-none gap-3 rounded-xl border border-transparent border-b-divider/70 py-5 pl-6 pr-4 transition-colors duration-200 hover:border-highlight/20 hover:bg-highlight/5 last:border-b-transparent sm:grid-cols-[minmax(0,1fr)_auto] sm:gap-x-8 sm:py-6"
        >
            <span
                aria-hidden="true"
                class="pointer-events-none absolute left-0 top-6 size-2.5 rounded-full border-2 border-highlight bg-card shadow-[0_0_0_4px_rgba(32,166,158,0.12)]"
            />
            <div class="min-w-0 space-y-1.5">
                <h3 class="!m-0 text-lg font-semibold tracking-tight text-ink sm:text-xl">
                    {&item.role}
                </h3>
                <p class="!m-0 text-sm font-medium text-highlight">
                    {&item.company}
                    if !item.location.is_empty() {
                        <span class="font-normal text-subtle">
                            {format!(" · {}", item.location)}
                        </span>
                    }
                </p>
                <ExperienceMetadata
                    employment_type={item.employment_type.clone()}
                    work_mode={item.work_mode.clone()}
                />
                if props.detailed && !item.responsibilities.is_empty() {
                    <ul
                        data-component="experience-responsibilities"
                        class="!my-4 list-disc space-y-2 pl-5 text-sm leading-relaxed text-subtle sm:text-base"
                    >
                        {for item.responsibilities.iter().map(|responsibility| html! {
                            <li>{responsibility}</li>
                        })}
                    </ul>
                }
            </div>
            <span class="select-none text-sm font-semibold text-highlight sm:whitespace-nowrap sm:text-right">
                {&item.period}
            </span>
        </article>
    }
}

#[derive(Properties, PartialEq)]
struct ExperienceMetadataProps {
    employment_type: String,
    work_mode: String,
}

#[function_component(ExperienceMetadata)]
fn experience_metadata(props: &ExperienceMetadataProps) -> Html {
    let values = [&props.employment_type, &props.work_mode]
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.as_str())
        .collect::<Vec<_>>();
    if values.is_empty() {
        return Html::default();
    }
    html! {
        <p data-component="experience-metadata" class="!m-0 text-xs text-subtle sm:text-sm">
            {values.join(" · ")}
        </p>
    }
}

#[function_component(ExperienceSection)]
pub fn experience_section(props: &ExperienceSectionProps) -> Html {
    html! {
        <section data-component="experience-section" class="section relative py-10 sm:py-14">
            <div data-component="section-heading" class="flex flex-wrap items-center justify-between gap-4">
                <h2 class="!m-0 select-none text-2xl font-semibold tracking-tight sm:text-3xl">
                    {if props.detailed { "Work experience" } else { "Experience" }}
                </h2>
                if !props.detailed {
                    <Link<Route>
                        to={Route::About}
                        classes="inline-flex items-center gap-2 text-sm font-medium text-highlight hover:underline"
                    >
                        {"View background"}
                        <Icon data={IconData::LUCIDE_MOUNTAIN} width="16" height="16"/>
                    </Link<Route>>
                }
            </div>
            <div data-component="experience-list" class="relative mt-5 border-l border-divider/70 ml-1">
                {for props.items.iter().cloned().enumerate().map(|(index, item)| html! {
                    <ExperienceEntry {item} {index} detailed={props.detailed}/>
                })}
            </div>
        </section>
    }
}
