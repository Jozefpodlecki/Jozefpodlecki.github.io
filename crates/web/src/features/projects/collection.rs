use super::cards::{ProjectCard, ProjectFilters};
use super::state::ProjectsContext;
use crate::{state::Load, content::Project};
use crate::features::motion::MotionReveal;
use crate::features::recovery::{ContentRecovery, RecoveryScope};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CollectionProps {
    #[prop_or_default]
    pub filterable: bool,
    #[prop_or_default]
    pub limit: usize,
}

#[function_component(ProjectCollection)]
pub fn project_collection(props: &CollectionProps) -> Html {
    let selected_tag = use_state(|| String::from("All"));
    let projects = use_context::<ProjectsContext>().expect("ProjectsProvider missing");

    match &projects.state {
        Load::Loading => html! {
            <p role="status" data-component="projects-loading">{"Loading projects..."}</p>
        },
        Load::Error(error) => html! {
            <div data-component="projects-error">
                <ContentRecovery
                    scope={RecoveryScope::Projects}
                    error={error.clone()}
                    on_retry={projects.retry.clone()}
                />
            </div>
        },
        Load::Ready(list) if list.is_empty() => html! {
            <div data-component="empty-projects" class="rounded-xl border border-divider bg-card p-8">
                <h2>{"Projects are being curated"}</h2>
                <p>{"Check back for published case studies."}</p>
            </div>
        },
        Load::Ready(list) => html! {
            <ProjectResults
                projects={list.clone()}
                selected={(*selected_tag).clone()}
                on_select={Callback::from(move |tag: String| selected_tag.set(tag))}
                filterable={props.filterable}
                limit={props.limit}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
struct ProjectResultsProps {
    projects: Vec<Project>,
    selected: String,
    on_select: Callback<String>,
    filterable: bool,
    limit: usize,
}

#[function_component(ProjectResults)]
fn project_results(props: &ProjectResultsProps) -> Html {
    let page = use_state(|| 1_usize);
    let mut tags: Vec<String> = props.projects.iter().flat_map(|project| project.tags.iter().cloned()).collect();
    tags.sort_unstable();
    tags.dedup();

    let filtered: Vec<Project> = props.projects.iter()
        .filter(|project| props.selected == "All" || project.tags.contains(&props.selected))
        .cloned()
        .collect();

    let paginate = props.filterable && props.limit == 0;
    let page_count = if paginate { filtered.len().div_ceil(super::pagination::PAGE_SIZE) } else { 1 };
    let current_page = (*page).min(page_count.max(1));
    let start = if paginate { (current_page - 1) * super::pagination::PAGE_SIZE } else { 0 };
    let visible: Vec<Project> = filtered.iter()
        .skip(start)
        .take(if paginate { super::pagination::PAGE_SIZE } else if props.limit == 0 { usize::MAX } else { props.limit })
        .cloned()
        .collect();

    let change_filter = {
        let page = page.clone();
        let on_select = props.on_select.clone();
        Callback::from(move |tag: String| {
            page.set(1);
            on_select.emit(tag);
        })
    };
    let change_page = {
        let page = page.clone();
        Callback::from(move |next: usize| page.set(next))
    };

    html! {
        <section data-component="project-results">
            if props.filterable {
                <ProjectFilters tags={tags} selected={props.selected.clone()} on_select={change_filter}/>
            }
            <MotionReveal delay_ms={60}>
                <div data-component="project-grid" class="grid grid-cols-1 gap-5 pb-12 md:grid-cols-2 xl:grid-cols-3">
                    {for visible.iter().cloned().map(|project| {
                        let slug = project.slug.clone();

                        html! {
                            <ProjectCard
                                key={slug}
                                project={project}
                            />
                        }
                    })}
                </div>
            </MotionReveal>
            if paginate && page_count > 1 {
                <super::pagination::ProjectPagination
                    current_page={current_page}
                    page_count={page_count}
                    total={filtered.len()}
                    on_change={change_page}
                />
            }
            if visible.is_empty() {
                <p role="status">{"No projects match this filter."}</p>
            }
        </section>
    }
}
