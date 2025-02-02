use log::debug;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

use crate::models::{FetchError, Summary};

pub async fn get_summary()  -> Result<Summary, FetchError> {
    let base_url = if cfg!(debug_assertions) {
        "http://localhost.:1420"
    } else {
        "https://jozefpodlecki.github.io"
    };

    let url = &format!("{}{}", base_url, "/public/summary.json");
    
    let request_options = RequestInit::new();
    request_options.set_method("GET");
    request_options.set_mode(RequestMode::NoCors);

    let headers = Headers::new().unwrap();
    headers.set("Cache-Control", "no-cache").unwrap();
    request_options.set_headers(&headers);

    let request = Request::new_with_str_and_init(url, &request_options)?;

    let window = gloo::utils::window();
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_value.dyn_into().unwrap();

    let js_value = JsFuture::from(response.json()?).await?;
    let summary: Summary = serde_wasm_bindgen::from_value(js_value).unwrap();
    debug!("{:?}", summary);

    Ok(summary)
}