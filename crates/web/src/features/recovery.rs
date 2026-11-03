use crate::content::ContentError;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(Clone, PartialEq)]
pub enum RecoveryScope {
    Essential,
    Projects,
}

#[derive(Properties, PartialEq)]
pub struct RecoveryProps {
    pub scope: RecoveryScope,
    pub error: ContentError,
    pub on_retry: Callback<MouseEvent>,
}

#[function_component(ContentRecovery)]
pub fn content_recovery(props: &RecoveryProps) -> Html {
    let essential = props.scope == RecoveryScope::Essential;
    let (title, description) = if essential {
        ("Couldn't open the portfolio", "The page couldn't load its essential information. You can try again without reloading.")
    } else {
        ("Projects aren't available right now", "The rest of the site is available. Try loading the projects again.")
    };
    let source = if essential { "core" } else { "projects" };
    let error_kind = props.error.category();
    let retry_label = if props.error.is_transient() { "Try again" } else { "Retry loading" };
    let layout = if essential {
        "mx-auto flex min-h-[65vh] w-full max-w-xl items-center justify-center px-5 py-16"
    } else {
        "w-full py-2"
    };
    html! {
        <div class={layout} data-component="content-recovery" data-error-source={source} data-error-kind={error_kind}>
            <section class="flex w-full flex-col gap-4 rounded-2xl border border-divider bg-card/90 p-6 shadow-lg shadow-black/5 backdrop-blur-lg sm:p-8" role="alert">
                <div class="flex items-center gap-3">
                    <span class="grid size-11 shrink-0 place-items-center rounded-xl border border-divider bg-layer text-highlight" aria-hidden="true">
                        <Icon data={IconData::LUCIDE_ALERT_TRIANGLE} width="22" height="22"/>
                    </span>
                    <div class="min-w-0">
                        <p class="!mb-1 text-[0.76rem] font-bold uppercase tracking-[0.23em] text-highlight">{"CONTENT UNAVAILABLE"}</p>
                        <h2 class="!m-0 !text-xl !leading-tight sm:!text-2xl">{title}</h2>
                    </div>
                </div>
                <p class="!m-0 max-w-[60ch] text-sm leading-relaxed text-subtle sm:text-base">{description}</p>
                <div class="flex flex-wrap items-center gap-3">
                    <button type="button" data-component="content-retry" class="inline-flex min-h-10 items-center justify-center gap-2 rounded-lg bg-highlight px-5 py-2 font-semibold text-on-accent transition-opacity hover:opacity-90 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-highlight" onclick={props.on_retry.clone()}>
                        <Icon data={IconData::LUCIDE_REFRESH_CW} width="17" height="17"/>
                        {retry_label}
                    </button>
                    if !essential {
                        <span class="text-xs text-subtle">{"Other sections remain available"}</span>
                    }
                </div>
            </section>
        </div>
    }
}
