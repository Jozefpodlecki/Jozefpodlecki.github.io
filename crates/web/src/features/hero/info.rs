use yew::prelude::*;
use yew_icons::{Icon, IconData};

use super::field::FlowMode;

#[derive(Properties, PartialEq)]
pub(super) struct FlowFieldInfoProps {
    pub mode: FlowMode,
    pub on_mode_change: Callback<FlowMode>,
}

#[function_component(FlowFieldInfo)]
pub(super) fn flow_field_info(props: &FlowFieldInfoProps) -> Html {
    let open = use_state(|| false);

    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    let dismiss = {
        let open = open.clone();
        Callback::from(move |_| open.set(false))
    };

    let onkeydown = {
        let open = open.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Escape" && *open {
                event.prevent_default();
                open.set(false);
            }
        })
    };

    let set_calm = {
        let callback = props.on_mode_change.clone();
        Callback::from(move |_| callback.emit(FlowMode::Calm))
    };

    let set_dynamic = {
        let callback = props.on_mode_change.clone();
        Callback::from(move |_| callback.emit(FlowMode::Dynamic))
    };

    html! {
        <>
            if *open {
                <button
                    data-component="hero-flow-info-dismiss"
                    type="button"
                    tabindex="-1"
                    aria-label="Close flow field information"
                    onclick={dismiss}
                    class="fixed inset-0 z-30 cursor-default bg-transparent"
                />
            }
            <div
                data-component="hero-flow-info"
                class="relative z-40 mt-2 flex flex-col items-end"
                onkeydown={onkeydown}
            >
                <div class="flex w-full flex-wrap items-center justify-end gap-2">
                    <div
                        data-component="hero-flow-mode-controls"
                        role="group"
                        aria-label="Flow field animation speed"
                        class="inline-flex select-none items-center rounded-full border border-divider bg-card/80 p-0.5"
                    >
                        <button
                            type="button"
                            data-component="hero-flow-mode"
                            data-mode="calm"
                            aria-pressed={(props.mode == FlowMode::Calm).to_string()}
                            onclick={set_calm}
                            class={if props.mode == FlowMode::Calm {
                                "inline-flex items-center gap-1.5 rounded-full bg-highlight px-3 py-1 text-xs font-semibold text-on-accent transition-colors duration-200"
                            } else {
                                "inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-medium text-subtle transition-colors duration-200 hover:text-highlight"
                            }}
                        >
                            <Icon data={IconData::LUCIDE_WAVES} width="14" height="14" />
                            {"Calm"}
                        </button>
                        <button
                            type="button"
                            data-component="hero-flow-mode"
                            data-mode="dynamic"
                            aria-pressed={(props.mode == FlowMode::Dynamic).to_string()}
                            onclick={set_dynamic}
                            class={if props.mode == FlowMode::Dynamic {
                                "inline-flex items-center gap-1.5 rounded-full bg-highlight px-3 py-1 text-xs font-semibold text-on-accent transition-colors duration-200"
                            } else {
                                "inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-medium text-subtle transition-colors duration-200 hover:text-highlight"
                            }}
                        >
                            <Icon data={IconData::LUCIDE_ZAP} width="14" height="14" />
                            {"Dynamic"}
                        </button>
                    </div>
                    <div
                        data-component="hero-flow-info-anchor"
                        class="relative shrink-0"
                    >
                        if *open {
                            <div
                                id="hero-flow-info-details"
                                data-component="hero-flow-info-details"
                                role="region"
                                aria-label="About the animated flow field"
                                class="absolute bottom-full right-0 z-50 mb-3 w-[min(24rem,calc(100vw-2rem))] rounded-xl border border-divider bg-card p-4 text-sm text-ink shadow-xl backdrop-blur-xl"
                            >
                                <h2 class="!m-0 !text-base !font-semibold !tracking-normal">
                                    {"A little mathematics in motion"}
                                </h2>
                                <p class="mb-0 mt-2 leading-relaxed text-subtle">
                                    {"This illustration is drawn live with the Canvas 2D API. Particles follow a mathematical vector field, forming flowing paths around two vortices. Move your pointer over the canvas to influence their direction."}
                                </p>
                                <p class="mb-0 mt-2 leading-relaxed text-subtle">
                                    {"Each page load starts with a different arrangement. Choose Calm or Dynamic to control the flow speed. Animation pauses in background tabs and becomes static when reduced motion is preferred."}
                                </p>
                            </div>
                        }
                        <button
                            data-component="hero-flow-info-toggle"
                            type="button"
                            aria-label="What is this flow field?"
                            aria-expanded={open.to_string()}
                            aria-controls="hero-flow-info-details"
                            onclick={toggle}
                            class="inline-flex select-none items-center gap-2 rounded-full border border-divider bg-card/80 px-3 py-1.5 text-xs font-medium text-subtle transition-colors hover:border-highlight hover:text-highlight focus-visible:outline-2 focus-visible:outline-highlight"
                        >
                            <Icon data={IconData::LUCIDE_INFO} width="15" height="15" />
                            {"What is this?"}
                        </button>
                    </div>
                </div>
            </div>
        </>
    }
}
