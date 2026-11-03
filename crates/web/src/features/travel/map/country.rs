use super::super::CountryShape;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct CountryPathProps {
    pub country: CountryShape,
    pub visited: bool,
    pub transit: bool,
    pub on_hover: Callback<Option<String>>,
}

#[function_component(CountryPath)]
pub(super) fn country_path(props: &CountryPathProps) -> Html {
    let country = &props.country;
    let visited = props.visited;
    let transit = props.transit;

    let fill = if visited {
        "var(--map-visited)"
    } else if transit {
        "var(--map-transit)"
    } else {
        "var(--map-land)"
    };

    if !visited && !transit {
        return html! {
            <path
                data-country={country.code.clone()}
                data-visited="false"
                data-transit="false"
                d={country.path.clone()}
                fill={fill}
                stroke="var(--map-stroke)"
                stroke-width="0.65"
                class="pointer-events-none"
                aria-hidden="true"
            />
        };
    }

    let code = country.code.clone();
    let on_hover_enter = props.on_hover.clone();
    let on_hover_leave = props.on_hover.clone();
    let on_focus = props.on_hover.clone();
    let on_blur = props.on_hover.clone();
    let on_key_select = props.on_hover.clone();
    let key_code = code.clone();
    let hover_code = code.clone();
    let focus_code = code.clone();

    html! {
        <path
            data-country={code.clone()}
            data-visited={visited.to_string()}
            data-transit={transit.to_string()}
            d={country.path.clone()}
            fill={fill}
            stroke="var(--map-stroke)"
            stroke-width="0.65"
            class="cursor-pointer transition-colors hover:fill-[var(--map-hover)] focus:outline-2 focus:outline-highlight focus:fill-[var(--map-selected)]"
            role="button"
            tabindex="0"
            aria-label={if visited {
                format!("Show travel details for {}", country.name)
            } else {
                format!("Show airport transit details for {}", country.name)
            }}
            onclick={Callback::from({
                let on_click = props.on_hover.clone();
                let click_code = code.clone();
                move |event: MouseEvent| {
                    event.stop_propagation();
                    on_click.emit(Some(click_code.clone()));
                }
            })}
            onmouseenter={Callback::from(move |_| on_hover_enter.emit(Some(hover_code.clone())))}
            onmouseleave={Callback::from(move |_| on_hover_leave.emit(None))}
            onfocus={Callback::from(move |_| on_focus.emit(Some(focus_code.clone())))}
            onblur={Callback::from(move |_| on_blur.emit(None))}
            onkeydown={Callback::from(move |event: KeyboardEvent| {
                if event.key() == "Enter" || event.key() == " " {
                    event.prevent_default();
                    on_key_select.emit(Some(key_code.clone()));
                }
            })}
        >
            <title>{&country.name}</title>
        </path>
    }
}
