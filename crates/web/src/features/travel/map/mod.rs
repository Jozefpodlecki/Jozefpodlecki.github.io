use super::{CountryShape, TravelKind, TravelRecord};
mod country;
mod tooltip;

use self::country::CountryPath;
use self::tooltip::TravelTooltip;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct WorldMapProps {
    pub countries: Vec<CountryShape>,
    pub visits: Vec<TravelRecord>,
    pub hovered: Option<String>,
    pub on_hover: Callback<Option<String>>,
    #[prop_or_default]
    pub expanded: bool,
}

#[function_component(WorldMap)]
pub(super) fn world_map(props: &WorldMapProps) -> Html {
    let hovered_visit = props.hovered.as_ref().and_then(|code| {
        props.countries.iter().find(|country| &country.code == code).and_then(|country| {
            let records: Vec<&TravelRecord> = props
                .visits
                .iter()
                .filter(|visit| &visit.code == code)
                .collect();
            (!records.is_empty()).then_some((country, records))
        })
    });

    html! {
        <div data-component="map-surface" class={if props.expanded {
                "relative h-full min-h-0 w-full select-none overflow-hidden bg-[var(--map-ocean)] transition-colors duration-200"
            } else {
                "relative min-w-0 select-none overflow-hidden rounded-xl bg-[var(--map-ocean)] transition-colors duration-200"
            }}>
            <svg
                data-component="world-map"
                viewBox="0 0 960 480"
                role="group"
                aria-label="Map of places visited"
                class={if props.expanded {
                    "block h-full w-full"
                } else {
                    "block w-full"
                }}
                xmlns="http://www.w3.org/2000/svg"
                onclick={Callback::from({
                    let on_hover = props.on_hover.clone();
                    move |_| on_hover.emit(None)
                })}
            >
                {for props.countries.iter().cloned().map(|country| {
                    let key = country.code.clone();
                    let visited = props.visits.iter().any(|visit| {
                        visit.code == country.code && visit.kind == TravelKind::Visit
                    });
                    let transit = props.visits.iter().any(|visit| {
                        visit.code == country.code && visit.kind == TravelKind::Transit
                    });
                    html! {
                        <CountryPath
                            key={key}
                            country={country}
                            visited={visited}
                            transit={transit}
                            on_hover={props.on_hover.clone()}
                        />
                    }
                })}
            </svg>
            if let Some((country, records)) = hovered_visit {
                <TravelTooltip
                    country={country.clone()}
                    records={records.into_iter().cloned().collect::<Vec<_>>()}
                    expanded={props.expanded}
                />
            }
        </div>
    }
}
