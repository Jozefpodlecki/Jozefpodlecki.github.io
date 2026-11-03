mod map;

use crate::content::fetch_json;
use self::map::WorldMap;
use serde::Deserialize;
use yew::prelude::*;
use yew_icons::{Icon, IconData};
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq, Deserialize)]
struct CountryShape {
    code: String,
    name: String,
    path: String,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TravelKind {
    #[default]
    Visit,
    Transit,
}

#[derive(Clone, PartialEq, Deserialize)]
struct TravelRecord {
    code: String,
    #[serde(default)]
    kind: TravelKind,
    #[serde(default)]
    year: Option<u16>,
    #[serde(default)]
    note: String,
}

#[derive(Clone, PartialEq)]
enum TravelState {
    Loading,
    Ready(Vec<TravelRecord>),
    Failed,
}


fn focus_node(node: &web_sys::Node) {
    let value: &wasm_bindgen::JsValue = node.as_ref();
    if let Ok(method) = js_sys::Reflect::get(value, &"focus".into()) {
        if let Ok(method) = method.dyn_into::<js_sys::Function>() {
            let _ = method.call0(value);
        }
    }
}

#[function_component(TravelMap)]
pub fn travel_map() -> Html {
    let state = use_state(|| TravelState::Loading);
    let retry = use_state(|| 0u32);
    let hovered = use_state(|| None::<String>);
    let expanded = use_state(|| false);
    let expand_ref = use_node_ref();
    let dialog_ref = use_node_ref();

    {
        let dialog_ref = dialog_ref.clone();
        use_effect_with(*expanded, move |is_expanded| {
            if *is_expanded {
                if let Some(dialog) = dialog_ref.cast::<web_sys::Element>() {
                    if let Ok(Some(close)) = dialog.query_selector("[data-component=travel-collapse]") {
                        focus_node(close.unchecked_ref());
                    }
                }
            }
            || ()
        });
    }
    let countries: Vec<CountryShape> = serde_json::from_str(include_str!("../world_countries.json"))
        .unwrap_or_default();

    {
        let state = state.clone();
        use_effect_with(*retry, move |_| {
            state.set(TravelState::Loading);
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_json::<Vec<TravelRecord>>("travel.json").await {
                    Ok(records) => state.set(TravelState::Ready(records)),
                    Err(error) => {
                        log::warn!("Travel content could not load: {error}");
                        state.set(TravelState::Failed);
                    }
                }
            });
            || ()
        });
    }

    let records = match &*state {
        TravelState::Ready(records) => records.clone(),
        _ => Vec::new(),
    };

    let retry_load = {
        let retry = retry.clone();
        Callback::from(move |_| retry.set((*retry).wrapping_add(1)))
    };
    let on_hover = {
        let hovered = hovered.clone();
        Callback::from(move |code| hovered.set(code))
    };
    let open_map = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(true))
    };
    let close_map = {
        let expanded = expanded.clone();
        let expand_ref = expand_ref.clone();
        Callback::from(move |_| {
            expanded.set(false);
            if let Some(button) = expand_ref.cast::<web_sys::HtmlElement>() {
                let _ = button.focus();
            }
        })
    };
    let dialog_keys = {
        let close_map = close_map.clone();
        let dialog_ref = dialog_ref.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Escape" {
                event.prevent_default();
                close_map.emit(());
                return;
            }

            if event.key() != "Tab" {
                return;
            }

            let Some(dialog) = dialog_ref.cast::<web_sys::Element>() else {
                return;
            };
            let Ok(focusable) = dialog.query_selector_all(
                "[data-component=world-map] [tabindex='0'], [data-component=travel-collapse]"
            ) else {
                return;
            };
            if focusable.length() == 0 {
                return;
            }

            let Some(first) = focusable.item(0) else {
                return;
            };
            let Some(last) = focusable.item(focusable.length() - 1) else {
                return;
            };
            let active = dialog.owner_document().and_then(|document| document.active_element());
            let active = active.as_ref().map(|element| element.unchecked_ref::<web_sys::Node>());
            let is_first = active.is_some_and(|node| node.is_same_node(Some(&first)));
            let is_last = active.is_some_and(|node| node.is_same_node(Some(&last)));
            let dialog_node: &web_sys::Node = dialog.unchecked_ref();
            let is_outside = active.is_none_or(|node| !dialog_node.contains(Some(node)));

            if event.shift_key() && (is_first || is_outside) {
                event.prevent_default();
                focus_node(&last);
            } else if !event.shift_key() && (is_last || is_outside) {
                event.prevent_default();
                focus_node(&first);
            }
        })
    };

    html! {
        <section data-component="travel-section" class="section relative py-[50px] pb-[70px]" aria-label="Travel map">
            <div class="mb-[22px] flex flex-wrap items-center justify-between gap-[15px] [&>h2]:m-0">
                <h2>{"Places I've been"}</h2>
                <button
                    data-component="travel-expand"
                    ref={expand_ref}
                    type="button"
                    aria-label="Expand travel map"
                    aria-expanded={expanded.to_string()}
                    aria-controls="travel-map-dialog"
                    onclick={open_map}
                    class="inline-flex select-none items-center gap-2 rounded-lg border border-divider bg-card/70 px-3 py-2 text-sm font-medium text-ink transition-colors hover:border-highlight hover:text-highlight"
                >
                    <Icon data={IconData::LUCIDE_ARROW_UP_RIGHT} width="16" height="16" />
                    {"Expand map"}
                </button>
            </div>
            <div data-component="travel-map" class="overflow-hidden rounded-2xl border border-divider bg-card/70 p-2 backdrop-blur-md sm:p-4">
                <WorldMap
                    countries={countries.clone()}
                    visits={records.clone()}
                    hovered={(*hovered).clone()}
                    on_hover={on_hover.clone()}
                />
                if matches!(&*state, TravelState::Failed) {
                    <div data-component="travel-recovery" class="mt-3 flex flex-wrap items-center gap-3 text-sm text-subtle">
                        <span>{"Travel information is temporarily unavailable."}</span>
                        <button
                            type="button"
                            onclick={retry_load}
                            class="rounded-lg border border-divider px-3 py-2 text-ink hover:border-highlight"
                        >
                            {"Try again"}
                        </button>
                    </div>
                }
            </div>
            if *expanded {
                <div
                    data-component="travel-map-dialog"
                    ref={dialog_ref}
                    id="travel-map-dialog"
                    role="dialog"
                    aria-modal="true"
                    aria-label="Expanded travel map"
                    class="fixed inset-0 z-[80] flex items-stretch justify-stretch p-3 sm:p-5"
                    onkeydown={dialog_keys}
                >
                    <button
                        type="button"
                        data-component="travel-dialog-backdrop"
                        tabindex="-1"
                        aria-label="Close expanded map"
                        onclick={Callback::from({
                            let close_map = close_map.clone();
                            move |_| close_map.emit(())
                        })}
                        class="absolute inset-0 cursor-default select-none bg-black/85 backdrop-blur-lg"
                    />
                    <div
                        data-component="travel-dialog-content"
                        class="relative z-10 h-full min-h-0 w-full overflow-hidden rounded-2xl border border-divider bg-card shadow-2xl"
                    >
                        <WorldMap
                            countries={countries.clone()}
                            visits={records.clone()}
                            hovered={(*hovered).clone()}
                            on_hover={on_hover}
                            expanded={true}
                        />
                        <button
                            type="button"
                            data-component="travel-collapse"
                            aria-label="Close expanded map"
                            onclick={Callback::from(move |_| close_map.emit(()))}
                            class="absolute right-3 top-3 z-20 grid size-11 select-none place-items-center rounded-full border border-divider bg-card/95 text-ink shadow-lg backdrop-blur-md transition-colors hover:border-highlight hover:text-highlight focus-visible:outline-2 focus-visible:outline-highlight sm:right-5 sm:top-5"
                        >
                            <Icon data={IconData::LUCIDE_X} width="20" height="20" />
                        </button>
                    </div>
                </div>
            }
        </section>
    }
}
