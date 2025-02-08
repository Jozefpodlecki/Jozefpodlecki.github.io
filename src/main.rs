mod app;
mod app_state;
mod icons;
mod experience;
mod projects_section;
mod projects;
mod api;
mod models;
mod dark_mode_toggle;
mod loader;
mod home;
mod not_found;
mod route;
mod breadcrumbs;
mod error;
mod layout;

use app::App;
use log::Level;

fn main() {
    let log_level = if cfg!(debug_assertions) {
        Level::Debug
    } else {
        Level::Info
    };

    wasm_logger::init(wasm_logger::Config::new(log_level));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
