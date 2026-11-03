use super::{connection::ConnectionNotice, footer::SiteFooter, navigation::PrimaryNavigation, photo::PhotoInfo};
use crate::features::{
    portfolio::{About, Home},
    projects::{ProjectPage, Projects},
};
use crate::routes::Route;
use crate::features::projects::state::ProjectsProvider;
use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct ShellProps {
    pub dark: bool,
    pub toggle: Callback<MouseEvent>,
}

#[function_component(BrowserShell)]
pub fn browser_shell(props: &ShellProps) -> Html {
    let move_pointer = Callback::from(|event: PointerEvent| {
        if event.pointer_type() != "mouse" {
            return;
        }

        if let Some(element) = event
            .current_target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
        {
            let position = format!(
                "--pointer-x: {}px; --pointer-y: {}px;",
                event.client_x(),
                event.client_y()
            );
            let _ = element.set_attribute("style", &position);
        }
    });

    let skip_to_content = Callback::from(|event: MouseEvent| {
        event.prevent_default();
        let target = event
            .current_target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok());
        if let Some(main) = target
            .and_then(|element| element.owner_document())
            .and_then(|document| document.get_element_by_id("main-content"))
        {
            if let Some(main) = main.dyn_ref::<web_sys::HtmlElement>() {
                let _ = main.focus();
            }
        }
    });

    html! {
        <HashRouter>
            <ProjectsProvider>
                <div
                data-component="site-frame"
                class="relative isolate min-h-screen"
                onpointermove={move_pointer}
                data-context-menu-policy={if cfg!(debug_assertions) { "enabled" } else { "disabled" }}
                oncontextmenu={Callback::from(|event: MouseEvent| {
                    if !cfg!(debug_assertions) {
                        event.prevent_default();
                    }
                })}
            >
                <SiteBackground />
                <div data-component="pointer-glow" aria-hidden="true" />
                <div data-component="app-shell" class="relative z-[1] mx-auto w-full max-w-[1240px] px-5 min-[641px]:px-[clamp(20px,5vw,64px)]">
                    <a
                        data-component="skip-navigation"
                        href="#main-content"
                        onclick={skip_to_content}
                        class="fixed left-4 top-3 z-50 -translate-y-24 rounded-md bg-highlight px-4 py-2 font-semibold text-on-accent no-underline focus:translate-y-0"
                    >
                        {"Skip to content"}
                    </a>
                    <header
                        data-component="site-header"
                        class="flex min-h-[76px] flex-wrap items-center justify-between gap-3 border-b border-divider py-4 sm:flex-nowrap sm:gap-4"
                    >
                        <Link<Route>
                            to={Route::Home}
                            classes="brand-glow select-none text-2xl font-bold tracking-tight text-ink no-underline"
                        >
                            {"JP"}
                        </Link<Route>>
                        <PrimaryNavigation />
                        <ThemeToggle dark={props.dark} toggle={props.toggle.clone()} />
                    </header>
                    <div id="main-content" data-component="route-content" tabindex="-1">
                        <Switch<Route> render={switch} />
                    </div>
                    <SiteFooter />
                    <PhotoInfo />
                </div>
                <ConnectionNotice />
                </div>
            </ProjectsProvider>
        </HashRouter>
    }
}

#[function_component(ThemeToggle)]
fn theme_toggle(props: &ShellProps) -> Html {
    let label = if props.dark {
        "Switch to light mode"
    } else {
        "Switch to dark mode"
    };

    html! {
        <button
            data-component="theme-toggle"
            type="button"
            class="grid size-11 shrink-0 select-none place-items-center rounded-full border border-divider bg-card/35 text-ink transition-colors duration-200 hover:border-highlight hover:bg-highlight/10 hover:text-highlight focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-highlight"
            aria-label={label}
            aria-pressed={props.dark.to_string()}
            onclick={props.toggle.clone()}
        >
            if props.dark {
                <Icon data={IconData::LUCIDE_SUN} width="20" height="20" />
            } else {
                <Icon data={IconData::LUCIDE_MOON} width="20" height="20" />
            }
        </button>
    }
}

#[function_component(SiteBackground)]
fn site_background() -> Html {
    html! {
        <div
            data-component="site-background"
            class="site-background"
            aria-hidden="true"
        />
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Projects => html! { <Projects /> },
        Route::Project { slug } => html! { <ProjectPage {slug} /> },
        Route::NotFound => html! {
            <main data-component="not-found-page" class="min-h-[65vh] py-16">
                <h1>{"Page not found"}</h1>
                <Link<Route>
                    to={Route::Home}
                    classes="inline-flex items-center gap-2"
                >
                    <Icon data={IconData::LUCIDE_ARROW_LEFT} width="16" height="16" />
                    {"Return home"}
                </Link<Route>>
            </main>
        },
    }
}
