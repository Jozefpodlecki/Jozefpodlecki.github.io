mod fallback;

use self::fallback::ArtworkFallback;
use crate::content::Project;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ImageStatus {
    Loading,
    Ready,
    Unavailable,
}

impl ImageStatus {
    fn attribute(self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Ready => "ready",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ProjectArtworkProps {
    pub project: Project,

    #[prop_or_default]
    pub cover: bool,
}

#[function_component(ProjectArtwork)]
pub fn project_artwork(props: &ProjectArtworkProps) -> Html {
    let status = use_state(|| ImageStatus::Loading);

    let source = props.project.image.trim();
    let available = !source.is_empty() && *status != ImageStatus::Unavailable;
    let loaded = available && *status == ImageStatus::Ready;
    let state = if source.is_empty() {
        ImageStatus::Unavailable
    } else {
        *status
    };

    let onload = {
        let status = status.clone();
        Callback::from(move |_: Event| status.set(ImageStatus::Ready))
    };

    let onerror = {
        let status = status.clone();
        let name = props.project.name.clone();
        Callback::from(move |_: Event| {
            log::warn!("Project artwork unavailable: {name}");
            status.set(ImageStatus::Unavailable);
        })
    };

    let emblem = props
        .project
        .name
        .chars()
        .find(|character| character.is_alphanumeric())
        .map(|character| character.to_uppercase().collect::<String>())
        .unwrap_or_else(|| "P".to_owned());

    html! {
        <div
            data-component="project-artwork"
            data-image-status={state.attribute()}
            class="relative h-full w-full overflow-hidden bg-layer"
        >
            if state == ImageStatus::Loading {
                <div
                    data-component="project-artwork-loading"
                    aria-hidden="true"
                    class="absolute inset-0 bg-layer motion-safe:animate-pulse"
                />
            }
            if state == ImageStatus::Unavailable {
                <ArtworkFallback
                    name={props.project.name.clone()}
                    emblem={emblem.clone()}
                    cover={props.cover}
                />
            }
            if !source.is_empty() && *status != ImageStatus::Unavailable {
                <img
                    data-component={if props.cover { "project-cover-image" } else { "project-card-image" }}
                    src={source.to_owned()}
                    alt={format!("Illustration of {}", props.project.name)}
                    aria-hidden={(!loaded).to_string()}
                    draggable="false"
                    loading={if props.cover { "eager" } else { "lazy" }}
                    decoding="async"
                    onload={onload}
                    onerror={onerror}
                    class={classes!(
                        "absolute",
                        "inset-0",
                        "h-full",
                        "w-full",
                        "object-cover",
                        "transition-[opacity,transform]",
                        "duration-300",
                        "group-hover:scale-[1.04]",
                        if loaded { "opacity-100" } else { "opacity-0" },
                    )}
                />
            }
        </div>
    }
}
