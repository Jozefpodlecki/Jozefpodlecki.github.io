use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct ArtworkFallbackProps {
    pub name: String,
    pub emblem: String,
    pub cover: bool,
}

#[function_component(ArtworkFallback)]
pub(super) fn artwork_fallback(props: &ArtworkFallbackProps) -> Html {
    html! {
        <div
            data-component="project-artwork-fallback"
            role="img"
            aria-label={format!("Preview unavailable for {}", props.name)}
            class="absolute inset-0 flex select-none flex-col items-center justify-center gap-2 overflow-hidden bg-[radial-gradient(circle_at_72%_20%,rgba(53,190,181,0.16),transparent_60%)] px-4 text-center"
        >
            <span
                aria-hidden="true"
                class={classes!(
                    "grid",
                    "place-items-center",
                    "rounded-2xl",
                    "border",
                    "border-divider",
                    "bg-card/70",
                    "font-semibold",
                    "text-highlight",
                    if props.cover { "size-20" } else { "size-14" },
                    if props.cover { "text-4xl" } else { "text-2xl" },
                )}
            >
                {props.emblem.clone()}
            </span>
            <span class="text-xs font-medium text-subtle">
                {"Preview unavailable"}
            </span>
        </div>
    }
}
