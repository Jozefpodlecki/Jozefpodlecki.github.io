mod app;
mod icons;
mod experience;
mod projects;
mod api;
mod models;
mod dark_mode_toggle;
mod loader;

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
