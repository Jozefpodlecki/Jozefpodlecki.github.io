mod viewer;

use crate::content::SiteData;
use viewer::BackgroundPhotoViewer;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[function_component(PhotoInfo)]
pub(super) fn photo_info() -> Html {
    let data = use_context::<SiteData>().expect("SiteData provider");
    let open = use_state(|| false);
    let trigger_ref = use_node_ref();

    let show_photo = {
        let open = open.clone();
        Callback::from(move |_| open.set(true))
    };

    let close_photo = {
        let open = open.clone();
        Callback::from(move |()| open.set(false))
    };

    html! {
        <>
            <button
                ref={trigger_ref.clone()}
                data-component="photo-info-toggle"
                type="button"
                aria-label="About this place - view background photograph"
                aria-expanded={open.to_string()}
                aria-controls={if *open { Some("photo-viewer") } else { None }}
                onclick={show_photo}
                class="fixed bottom-3 right-3 z-20 hidden select-none items-center justify-center gap-1.5 rounded-full border border-divider bg-card/80 px-2.5 py-1.5 text-xs font-medium text-ink shadow-sm backdrop-blur-xl transition-colors hover:border-highlight hover:text-highlight focus-visible:outline-2 focus-visible:outline-highlight sm:bottom-5 sm:right-5 sm:inline-flex"
            >
                <Icon
                    data={IconData::LUCIDE_MAP_PIN}
                    width="14"
                    height="14"
                />
                <span>{"About this place"}</span>
            </button>
            if *open {
                <BackgroundPhotoViewer
                    details={data.profile.background.clone()}
                    on_close={close_photo}
                    trigger_ref={trigger_ref}
                />
            }
        </>
    }
}
