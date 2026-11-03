use crate::content::BackgroundPhoto;
use crate::env::AppEnvironment;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq)]
pub(super) struct BackgroundPhotoViewerProps {
    pub details: BackgroundPhoto,
    pub on_close: Callback<()>,
    pub trigger_ref: NodeRef,
}

#[function_component(BackgroundPhotoViewer)]
pub(super) fn background_photo_viewer(props: &BackgroundPhotoViewerProps) -> Html {
    let environment = use_context::<AppEnvironment>().expect("AppEnvironment provider");
    let show_entire_photo = use_state(|| false);
    let close_ref = use_node_ref();
    let fit_ref = use_node_ref();

    {
        let document = environment.document.clone();
        let close_ref = close_ref.clone();
        let trigger_ref = props.trigger_ref.clone();

        use_effect_with((), move |_| {
            if let Some(root) = document.document_element() {
                let _ = root.set_attribute("data-photo-viewer-open", "true");
            }
            if let Some(button) = close_ref.cast::<HtmlElement>() {
                let _ = button.focus();
            }

            move || {
                if let Some(root) = document.document_element() {
                    let _ = root.remove_attribute("data-photo-viewer-open");
                }
                if let Some(button) = trigger_ref.cast::<HtmlElement>() {
                    let _ = button.focus();
                }
            }
        });
    }

    let close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let toggle_fit = {
        let show_entire_photo = show_entire_photo.clone();
        Callback::from(move |_| show_entire_photo.set(!*show_entire_photo))
    };

    let onkeydown = {
        let on_close = props.on_close.clone();
        let close_ref = close_ref.clone();
        let fit_ref = fit_ref.clone();

        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Escape" {
                event.prevent_default();
                on_close.emit(());
            } else if event.key() == "Tab" {
                let Some(close_button) = close_ref.cast::<HtmlElement>() else {
                    return;
                };
                let Some(fit_button) = fit_ref.cast::<HtmlElement>() else {
                    return;
                };

                event.prevent_default();
                let close_focused = close_button.matches(":focus").unwrap_or(false);
                let focus_close = !close_focused;

                if focus_close {
                    let _ = close_button.focus();
                } else {
                    let _ = fit_button.focus();
                }
            }
        })
    };

    let photo_src = if props.details.image.is_empty() {
        "wallpaper.jpg".to_owned()
    } else {
        props.details.image.clone()
    };

    html! {
        <div
            id="photo-viewer"
            data-component="photo-viewer"
            data-photo-fit={if *show_entire_photo { "contain" } else { "cover" }}
            role="dialog"
            aria-modal="true"
            aria-label="Background photograph"
            class="photo-viewer fixed inset-0 z-[90] isolate overflow-hidden bg-[#071112] text-white"
            onkeydown={onkeydown}
        >
            <img
                data-component="photo-viewer-image"
                src={photo_src}
                alt=""
                aria-hidden="true"
                draggable="false"
                class={classes!(
                    "pointer-events-none",
                    "absolute",
                    "inset-0",
                    "h-full",
                    "w-full",
                    "select-none",
                    "transition-[object-position]",
                    "duration-300",
                    if *show_entire_photo { "object-contain" } else { "object-cover" },
                )}
            />
            <button
                data-component="photo-info-dismiss"
                type="button"
                tabindex="-1"
                aria-label="Close photograph viewer"
                class="absolute inset-0 z-10 cursor-default bg-transparent"
                onclick={close.clone()}
            />
            <div class="pointer-events-none absolute inset-x-0 bottom-0 z-10 h-48 bg-gradient-to-t from-black/75 to-transparent" />
            <div class="absolute right-4 top-4 z-20 flex flex-wrap justify-end gap-2 sm:right-6 sm:top-6">
                <button
                    ref={fit_ref}
                    data-component="photo-viewer-fit"
                    type="button"
                    aria-pressed={show_entire_photo.to_string()}
                    class="inline-flex min-h-11 select-none items-center gap-2 rounded-full border border-white/30 bg-black/50 px-4 py-2 text-sm font-medium text-white backdrop-blur-md transition-colors hover:bg-black/75 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white"
                    onclick={toggle_fit}
                >
                    <Icon data={IconData::LUCIDE_MAXIMIZE} width="16" height="16" />
                    {if *show_entire_photo { "Fill screen" } else { "Entire photo" }}
                </button>
                <button
                    ref={close_ref}
                    data-component="photo-viewer-close"
                    type="button"
                    class="inline-flex min-h-11 select-none items-center gap-2 rounded-full border border-white/30 bg-black/50 px-4 py-2 text-sm font-medium text-white backdrop-blur-md transition-colors hover:bg-black/75 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white"
                    onclick={close}
                >
                    <Icon data={IconData::LUCIDE_ARROW_LEFT} width="16" height="16" />
                    {"Back to portfolio"}
                </button>
            </div>
            <div
                data-component="photo-details"
                class="pointer-events-none absolute bottom-5 left-5 z-20 max-w-[min(34rem,calc(100vw-2.5rem))] text-white sm:bottom-8 sm:left-8"
            >
                <div class="flex items-center gap-2 text-sm text-white/80">
                    <Icon data={IconData::LUCIDE_MAP_PIN} width="17" height="17" />
                    <span>{&props.details.location}</span>
                </div>
                <h2 class="mb-1 mt-2 !text-xl !font-semibold !tracking-tight text-white sm:!text-2xl">
                    {&props.details.title}
                </h2>
                if let Some(year) = props.details.year {
                    <p class="m-0 text-sm text-white/80">{year}</p>
                }
                if !props.details.photographer.is_empty() {
                    <p class="m-0 mt-1 text-sm text-white/80">
                        {format!("Photo by {}", props.details.photographer)}
                    </p>
                }
                if !props.details.description.is_empty() {
                    <p class="m-0 mt-1 text-sm text-white/80">{&props.details.description}</p>
                }
            </div>
        </div>
    }
}
