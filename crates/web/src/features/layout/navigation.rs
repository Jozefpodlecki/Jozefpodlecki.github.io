use crate::routes::Route;
use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::*;

#[function_component(PrimaryNavigation)]
pub(super) fn primary_navigation() -> Html {
    let route = use_route::<Route>().unwrap_or(Route::Home);
    let items = [
        (Route::Home, "Home", "home", IconData::LUCIDE_HOME),
        (Route::About, "About", "about", IconData::LUCIDE_USER),
        (Route::Projects, "Projects", "projects", IconData::LUCIDE_FOLDER),
    ];

    html! {
        <nav
            data-component="site-navigation"
            aria-label="Primary navigation"
            class="order-3 flex w-full items-center justify-around gap-4 sm:order-none sm:w-auto sm:gap-8"
        >
            {for items.into_iter().map(|(destination, label, key, icon)| {
                let active = match (&route, &destination) {
                    (Route::Project { .. }, Route::Projects) => true,
                    _ => route == destination,
                };
                let classes = classes!(
                    "inline-flex",
                    "select-none",
                    "items-center",
                    "gap-1.5",
                    "no-underline",
                    "py-2",
                    "border-b-2",
                    "transition-colors",
                    "hover:text-highlight",
                    "hover:border-highlight",
                    "focus-visible:text-highlight",
                    "focus-visible:outline-2",
                    "focus-visible:outline-offset-4",
                    "focus-visible:outline-highlight",
                    if active { "border-highlight" } else { "border-transparent" },
                    if active { "font-semibold" } else { "font-normal" },
                    if active { "text-ink" } else { "text-subtle" },
                );

                html! {
                    <Link<Route> to={destination} classes={classes}>
                        <Icon data={icon} width="15" height="15" />
                        <span
                            data-component="navigation-item"
                            data-route={key}
                            data-active={active.to_string()}
                            aria-current={if active { Some("page") } else { None }}
                        >
                            {label}
                        </span>
                    </Link<Route>>
                }
            })}
        </nav>
    }
}
