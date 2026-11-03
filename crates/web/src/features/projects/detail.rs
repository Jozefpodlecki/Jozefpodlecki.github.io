use super::artwork::ProjectArtwork;
use crate::state::Load;
use crate::content::Project;
use crate::features::motion::MotionReveal;
use crate::features::recovery::{ContentRecovery, RecoveryScope};
use crate::routes::Route;
use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectProps {
    pub slug: String,
}

#[function_component(ProjectPage)]
pub fn project_page(props: &ProjectProps) -> Html {
    let projects = use_context::<super::state::ProjectsContext>()
        .expect("ProjectsProvider missing");

    html! {
        <main
            class="min-h-[65vh] pb-12 pt-8"
            data-component="project-detail-page"
        >
            <Link<Route>
                to={Route::Projects}
                classes="mb-8 inline-flex items-center gap-2 text-sm font-medium no-underline hover:underline"
            >
                <Icon data={IconData::LUCIDE_ARROW_LEFT} width="16" height="16" />
                {"All projects"}
            </Link<Route>>
            {match &projects.state {
                Load::Loading => html! {
                    <p role="status" class="py-12 text-subtle">{"Loading project..."}</p>
                },
                Load::Error(error) => html! {
                    <ContentRecovery
                        scope={RecoveryScope::Projects}
                        error={error.clone()}
                        on_retry={projects.retry.clone()}
                    />
                },
                Load::Ready(list) => match list.iter().find(|project| {
                    project.slug == props.slug && !project.slug.is_empty()
                }) {
                    Some(project) => html! { <ProjectCaseStudy project={project.clone()} /> },
                    None => html! {
                        <section data-component="project-not-found" class="py-12">
                            <h1>{"Project not found"}</h1>
                            <p class="text-subtle">{"This project is not in the current collection."}</p>
                        </section>
                    },
                },
            }}
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct ProjectCaseStudyProps {
    project: Project,
}

#[function_component(ProjectCaseStudy)]
fn project_case_study(props: &ProjectCaseStudyProps) -> Html {
    let project = &props.project;

    html! {
        <article
            data-component="project-case-study"
            data-project-slug={project.slug.clone()}
            class="flex flex-col gap-8 sm:gap-10"
        >
            <MotionReveal>
                <ProjectHeading project={project.clone()} />
            </MotionReveal>
            <MotionReveal delay_ms={90}>
                <ProjectCover project={project.clone()} />
            </MotionReveal>
            <div class="border-t border-divider pt-8">
                <ProjectNarrative project={project.clone()} />
            </div>
        </article>
    }
}

#[function_component(ProjectHeading)]
fn project_heading(props: &ProjectCaseStudyProps) -> Html {
    let project = &props.project;

    html! {
        <header data-component="project-heading" class="flex flex-col gap-5 sm:gap-6">
            <div class="flex flex-wrap items-center gap-2">
                {for project.tags.iter().map(|tag| html! {
                    <span
                        data-component="project-tag"
                        class="select-none rounded-full border border-divider bg-card/60 px-3 py-1 text-xs font-medium text-highlight"
                    >
                        {tag}
                    </span>
                })}
            </div>
            <h1 class="m-0 max-w-4xl text-4xl font-semibold tracking-tight text-ink sm:text-5xl lg:text-6xl">
                {&project.name}
            </h1>
            <p class="m-0 max-w-3xl text-base leading-relaxed text-subtle sm:text-xl">
                {&project.description}
            </p>
            <ProjectActions project={project.clone()} />
        </header>
    }
}

#[function_component(ProjectCover)]
fn project_cover(props: &ProjectCaseStudyProps) -> Html {
    html! {
        <figure
            data-component="project-cover"
            data-project-slug={props.project.slug.clone()}
            class="project-case-cover relative m-0 select-none aspect-[16/9] overflow-hidden rounded-2xl border border-divider sm:aspect-[21/9]"
        >
            <ProjectArtwork key={props.project.slug.clone()} project={props.project.clone()} cover={true} />
        </figure>
    }
}

#[function_component(ProjectNarrative)]
fn project_narrative(props: &ProjectCaseStudyProps) -> Html {
    let paragraphs = props.project.body.split("\n\n").filter(|text| !text.trim().is_empty());

    html! {
        <section
            data-component="case-study-body"
            aria-labelledby="project-overview-heading"
            class="mx-auto w-full max-w-4xl min-w-0"
        >
            <h2 id="project-overview-heading" class="mb-5 mt-0 text-2xl font-semibold text-ink">
                {"Overview"}
            </h2>
            <div class="flex w-full flex-col gap-5 text-base leading-8 text-subtle">
                {for paragraphs.map(|paragraph| html! {
                    <p class="m-0">{paragraph.trim()}</p>
                })}
            </div>
        </section>
    }
}

#[function_component(ProjectActions)]
fn project_actions(props: &ProjectCaseStudyProps) -> Html {
    let project = &props.project;

    if project.link.is_empty() && project.github.is_empty() {
        return Html::default();
    }

    html! {
        <nav
            data-component="project-actions"
            aria-label={format!("Links for {}", project.name)}
            class="flex flex-wrap gap-3"
        >
            if !project.link.is_empty() {
                <a
                    href={project.link.clone()}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="project-live-action inline-flex min-h-11 items-center justify-center gap-2 rounded-md bg-highlight px-4 py-2 font-semibold no-underline transition-opacity hover:opacity-90"
                    data-component="project-detail-live-link"
                >
                    <Icon data={IconData::LUCIDE_EXTERNAL_LINK} width="17" height="17" />
                    {"Live site"}
                </a>
            }
            if !project.github.is_empty() {
                <a
                    href={project.github.clone()}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex min-h-11 items-center justify-center gap-2 rounded-md border border-divider bg-card px-4 py-2 font-medium text-ink no-underline hover:border-highlight hover:text-highlight"
                    data-component="project-detail-repository-link"
                >
                    <Icon data={IconData::SIMPLE_ICONS_GITHUB} width="17" height="17" />
                    {"Source code"}
                </a>
            }
        </nav>
    }
}
