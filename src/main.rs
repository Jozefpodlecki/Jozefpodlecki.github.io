mod app;
mod app_state;
mod api;
mod components;
mod models;
mod pages;
mod route;

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
