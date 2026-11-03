mod app;
mod content;
mod env;
mod features;
mod routes;
mod state;

use app::App;
use env::AppEnvironment;

fn main() {
    console_error_panic_hook::set_once();
    let environment = AppEnvironment::new().expect("Could not initialize browser environment");
    wasm_logger::init(wasm_logger::Config::new(environment.log_level()));
    yew::Renderer::<App>::with_root_and_props(
        environment.root.clone(),
        app::AppProps { environment },
    ).render();
}
