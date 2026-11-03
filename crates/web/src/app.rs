use crate::content::{fetch_core, SiteData};
use crate::state::Load;
use crate::features::recovery::{ContentRecovery, RecoveryScope};
use crate::env::{AppEnvironment, MotionService, ThemeService};
use crate::features::layout::BrowserShell;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub environment: AppEnvironment,
}

fn hide_splash(document: &web_sys::Document) {
    if let Some(element) = document.get_element_by_id("splash") {
        let _ = element.set_attribute("class", "splash splash--hidden");
        gloo::timers::callback::Timeout::new(350, move || element.remove()).forget();
    }
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let core = use_state(|| Load::<SiteData>::Loading);
    let attempt = use_state(|| 0u32);
    let theme = ThemeService::new(props.environment.clone());
    let dark = use_state(|| theme.current_is_dark());

    {
        let core = core.clone();
        use_effect_with(*attempt, move |_| {
            core.set(Load::Loading);
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_core().await {
                    Ok(data) => core.set(Load::Ready(data)),
                    Err(error) => {
                        log::warn!("Core portfolio load failed: {error}");
                        core.set(Load::Error(error));
                    },
                }
            });
            || ()
        });
    }
    {
        let document = props.environment.document.clone();
        use_effect_with((*core).clone(), move |status| {
            if !matches!(status, Load::Loading) {
                hide_splash(&document);
            }
            || ()
        });
    }

    let retry = {
        let attempt = attempt.clone();
        Callback::from(move |_| attempt.set((*attempt).wrapping_add(1)))
    };
    let toggle = {
        let dark = dark.clone();
        Callback::from(move |_| {
            let next = !*dark;
            theme.set_dark(next);
            dark.set(next);
        })
    };

    match &*core {
        Load::Loading => html! { <div data-component="boot-placeholder" class="min-h-screen" aria-label="Loading portfolio" /> },
        Load::Error(error) => html! {
            <main data-component="content-error" class="min-h-[65vh]">
                <ContentRecovery scope={RecoveryScope::Essential} error={error.clone()} on_retry={retry.clone()}/>
            </main>
        },
        Load::Ready(data) => html! {
            <ContextProvider<AppEnvironment> context={props.environment.clone()}>
            <ContextProvider<MotionService> context={MotionService::new(props.environment.clone())}>
                <ContextProvider<SiteData> context={data.clone()}>
                    <BrowserShell dark={*dark} toggle={toggle}/>
                </ContextProvider<SiteData>>
            </ContextProvider<MotionService>>
            </ContextProvider<AppEnvironment>>
        },
    }
}
