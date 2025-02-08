use yew::*;
use yew_router::{HashRouter, Switch};

use crate::{api::get_summary, app_state::AppState, dark_mode_toggle::DarkModeToggle, error::ErrorPage, layout::Layout, loader::Loader, route::{switch, Route}};

async fn fetch_summary(
    app_state: UseStateHandle<AppState>, 
    has_error: UseStateHandle<bool>, 
    is_loading: UseStateHandle<bool>,
) {
    is_loading.set(true);
    has_error.set(false);

    match get_summary().await {
        Ok(summary) => {
            app_state.set(AppState { summary });
        }
        Err(_) => {
            has_error.set(true);
        }
    }

    is_loading.set(false);
}

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::default() );
    let has_error = use_state(|| false);
    let is_loading = use_state(|| true);

    {
        let has_error = has_error.clone();
        let is_loading = is_loading.clone();
        let app_state = app_state.clone();
        use_effect_with(
            (),
            move |_| {
                wasm_bindgen_futures::spawn_local(fetch_summary(
                    app_state.clone(),
                    has_error.clone(),
                    is_loading.clone(),
                ));
            },
        );
    }

    let reload = {
        let app_state = app_state.clone();
        let has_error = has_error.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(fetch_summary(
                app_state.clone(),
                has_error.clone(),
                is_loading.clone(),
            ));
        })
    };

    if *has_error {
            return html! { <ErrorPage {reload} /> }
    }

    if *is_loading {
        return html! {
            <>
                <div class="h-0 sticky top-1 right-1">
                    <DarkModeToggle/>
                </div>
                <div>
                    <Layout full_height_and_center={true}>
                        <Loader/>
                    </Layout>
                </div>
            </>
        }
    }

    html! {
        <ContextProvider<AppState> context={(*app_state).clone()}>
            <div class="h-0 sticky top-1 right-1">
                <DarkModeToggle/>
            </div>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        </ContextProvider<AppState>>
    }
}
