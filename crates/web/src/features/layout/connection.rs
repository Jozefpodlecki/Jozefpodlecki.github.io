use crate::env::AppEnvironment;
use gloo::{events::EventListener, timers::callback::Interval};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

fn check_connection(disconnected: UseStateHandle<bool>, dismissed: UseStateHandle<bool>) {
    wasm_bindgen_futures::spawn_local(async move {
        let path = format!("index.html?connection-check={}", js_sys::Date::now() as u64);
        let available = gloo::net::http::Request::get(&path)
            .send()
            .await
            .is_ok_and(|response| response.ok());

        if available {
            dismissed.set(false);
        }
        disconnected.set(!available);
    });
}

#[function_component(ConnectionNotice)]
pub(super) fn connection_notice() -> Html {
    let environment = use_context::<AppEnvironment>().expect("AppEnvironment provider missing");
    let disconnected = use_state(|| false);
    let dismissed = use_state(|| false);

    {
        let disconnected = disconnected.clone();
        let dismissed = dismissed.clone();
        let window = environment.window.clone();

        use_effect_with((), move |_| {
            let offline_state = disconnected.clone();
            let offline_dismissed = dismissed.clone();
            let offline = EventListener::new(&window, "offline", move |_| {
                offline_dismissed.set(false);
                offline_state.set(true);
            });

            let online_state = disconnected.clone();
            let online_dismissed = dismissed.clone();
            let online = EventListener::new(&window, "online", move |_| {
                check_connection(online_state.clone(), online_dismissed.clone());
            });

            let local = window
                .location()
                .hostname()
                .ok()
                .is_some_and(|hostname| hostname == "localhost" || hostname == "127.0.0.1");

            let interval = if local {
                let status = disconnected.clone();
                let dismissed = dismissed.clone();
                Some(Interval::new(7000, move || {
                    check_connection(status.clone(), dismissed.clone());
                }))
            } else {
                None
            };

            move || {
                drop(offline);
                drop(online);
                drop(interval);
            }
        });
    }

    let retry = {
        let disconnected = disconnected.clone();
        let dismissed = dismissed.clone();
        Callback::from(move |_| check_connection(disconnected.clone(), dismissed.clone()))
    };

    let dismiss = {
        let dismissed = dismissed.clone();
        Callback::from(move |_| dismissed.set(true))
    };

    if !*disconnected || *dismissed {
        return Html::default();
    }

    html! {
        <aside
            data-component="connection-notice"
            role="status"
            aria-live="polite"
            class="fixed bottom-20 left-4 right-4 z-[75] mx-auto max-w-md rounded-xl border border-divider bg-card p-4 text-ink shadow-2xl sm:bottom-6 sm:left-6 sm:right-auto"
        >
            <div class="flex items-start gap-3">
                <Icon data={IconData::LUCIDE_WIFI_OFF} width="20" height="20" />
                <div class="min-w-0 flex-1">
                    <p class="m-0 font-semibold">{"Connection unavailable"}</p>
                    <p class="mb-3 mt-1 text-sm leading-relaxed text-subtle">
                        {"The server cannot be reached. Existing pages remain visible, but new content and updates may not load."}
                    </p>
                    <button
                        type="button"
                        data-component="connection-retry"
                        onclick={retry}
                        class="rounded-md border border-divider px-3 py-1.5 text-sm font-medium text-ink transition-colors hover:border-highlight hover:text-highlight"
                    >
                        {"Try again"}
                    </button>
                </div>
                <button
                    type="button"
                    data-component="connection-dismiss"
                    aria-label="Dismiss connection notice"
                    onclick={dismiss}
                    class="grid size-8 shrink-0 place-items-center rounded-full text-subtle transition-colors hover:text-highlight"
                >
                    <Icon data={IconData::LUCIDE_X} width="17" height="17" />
                </button>
            </div>
        </aside>
    }
}
