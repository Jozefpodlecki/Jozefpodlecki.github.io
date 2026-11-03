use super::artwork::ProjectArtwork;
use crate::{content::Project, routes::Route};
use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    pub project: Project,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let project = &props.project;
    html! {
        <article
            data-component="project-card"
            data-project-slug={project.slug.clone()}
            class="group cursor-default select-none overflow-hidden rounded-xl border border-divider bg-card transition duration-200 hover:-translate-y-1 hover:border-highlight focus-within:border-highlight"
        >
            <div class="project-visual select-none h-40 overflow-hidden sm:h-44">
                <ProjectArtwork project={project.clone()} />
            </div>
            <div class="flex flex-col gap-3 p-5 sm:p-6">
                <p class="text-[0.67rem] font-semibold uppercase tracking-[0.15em] text-highlight">{project.tags.join(" / ")}</p>
                <h2 class="m-0 text-xl font-semibold tracking-tight">{&project.name}</h2>
                <p>{&project.description}</p>
                <ProjectLinks project={project.clone()} />
            </div>
        </article>
    }
}


#[derive(Properties, PartialEq)]
struct ProjectLinksProps {
    project: Project,
}

#[function_component(ProjectLinks)]
fn project_links(props: &ProjectLinksProps) -> Html {
    let project = &props.project;

    html! {
        <nav
            data-component="project-card-links"
            aria-label={format!("Links for {}", project.name)}
            class="mt-auto flex items-center gap-2 pt-2 text-sm font-medium"
        >
            if !project.slug.is_empty() {
                <Link<Route>
                    to={Route::Project { slug: project.slug.clone() }}
                    classes="inline-flex items-center gap-1.5 text-highlight no-underline transition-opacity hover:opacity-75 focus-visible:outline-2 focus-visible:outline-highlight"
                >
                    {"Read case study"}
                    <Icon data={IconData::LUCIDE_ARROW_RIGHT} width="16" height="16" />
                </Link<Route>>
            }
            if !project.github.trim().is_empty() {
                <a
                    data-component="project-repository-link"
                    href={project.github.clone()}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="ml-auto inline-grid size-8 shrink-0 place-items-center rounded-full border border-divider text-highlight no-underline transition-colors hover:border-highlight hover:bg-highlight/10 focus-visible:outline-2 focus-visible:outline-highlight"
                    aria-label={format!("Repository for {}", project.name)}
                    title="Source code"
                >
                    <Icon data={IconData::SIMPLE_ICONS_GITHUB} width="16" height="16" />
                </a>
            }
            if !project.link.trim().is_empty() {
                <a
                    data-component="project-live-link"
                    href={project.link.clone()}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-grid size-8 shrink-0 place-items-center rounded-full border border-divider text-highlight no-underline transition-colors hover:border-highlight hover:bg-highlight/10 focus-visible:outline-2 focus-visible:outline-highlight"
                    aria-label={format!("Live site for {}", project.name)}
                    title="Live site"
                >
                    <Icon data={IconData::LUCIDE_EXTERNAL_LINK} width="16" height="16" />
                </a>
            }
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct ProjectFiltersProps {
    pub tags: Vec<String>,
    pub selected: String,
    pub on_select: Callback<String>,
}

#[function_component(ProjectFilters)]
pub fn project_filters(props: &ProjectFiltersProps) -> Html {
    let options = std::iter::once(String::from("All")).chain(props.tags.iter().cloned());
    html! {
        <div
            data-component="project-filters"
            class="flex select-none flex-wrap gap-2 pb-6"
            role="group"
            aria-label="Filter projects by technology"
        >
            {for options.map(|tag| {
                let active = props.selected == tag;
                let on_select = props.on_select.clone();
                let selected_tag = tag.clone();
                html! {
                    <button
                        type="button"
                        data-component="project-filter"
                        data-filter-tag={tag.clone()}
                        class={classes!(
                            "rounded-full",
                            "border",
                            "px-3",
                            "py-1.5",
                            "text-xs",
                            "transition-colors",
                            "focus-visible:outline-2",
                            "focus-visible:outline-offset-2",
                            "focus-visible:outline-highlight",
                            if active { "border-highlight" } else { "border-divider" },
                            if active { "bg-highlight" } else { "bg-card" },
                            if active { "text-on-accent" } else { "text-ink" },
                            "hover:border-highlight",
                        )}
                        aria-pressed={active.to_string()}
                        onclick={Callback::from(move |_| on_select.emit(selected_tag.clone()))}
                    >
                        {tag}
                    </button>
                }
            })}
        </div>
    }
}
