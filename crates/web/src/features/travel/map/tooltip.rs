use super::super::{CountryShape, TravelKind, TravelRecord};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq)]
pub(super) struct TravelTooltipProps {
    pub country: CountryShape,
    pub records: Vec<TravelRecord>,
    pub expanded: bool,
}

#[function_component(TravelTooltip)]
pub(super) fn travel_tooltip(props: &TravelTooltipProps) -> Html {
    html! {
        <div
            data-component="travel-tooltip"
            role="status"
            aria-live="polite"
            class={if props.expanded {
                "pointer-events-none absolute left-4 top-4 z-10 max-h-[calc(100%-2rem)] max-w-[min(28rem,calc(100%-5rem))] overflow-y-auto rounded-xl border border-divider bg-card/95 px-4 py-3 text-sm text-ink shadow-xl backdrop-blur-md sm:left-6 sm:top-6"
            } else {
                "pointer-events-none absolute right-3 top-3 max-w-[min(19rem,70%)] rounded-lg border border-divider bg-card/95 px-3 py-2 text-sm text-ink shadow-xl backdrop-blur-md"
            }}
        >
            <strong class="block font-semibold">{&props.country.name}</strong>
            <ul class="mt-1 flex list-none flex-col gap-1 p-0 text-xs text-subtle">
                {for props.records.iter().map(|record| html! {
                    <li
                        data-component="travel-visit"
                        data-kind={if record.kind == TravelKind::Transit { "transit" } else { "visit" }}
                        class="flex items-start gap-1.5"
                    >
                        if record.kind == TravelKind::Transit {
                            <span class="shrink-0 text-highlight" aria-hidden="true">
                                <Icon data={IconData::LUCIDE_PLANE} width="14" height="14" />
                            </span>
                        }
                        <span>
                            {&record.note}
                            if let Some(year) = record.year {
                                <span class="ml-1 font-medium text-highlight">{format!("- {year}")}</span>
                            }
                        </span>
                    </li>
                })}
            </ul>
        </div>
    }
}
