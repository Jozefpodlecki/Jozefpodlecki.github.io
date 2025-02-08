use yew::*;
use yew_router::hooks::use_navigator;

use crate::{layout::Layout, route::Route};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();
    let on_home = Callback::from(move |_| navigator.push(&Route::Home));
    
    html! {
        <div>
            <Layout full_height_and_center={true}>
                <div class="flex-col-center">
                    <svg class="w-16 h-16 text-red-600 mb-6" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
                        <path d="M18 6L6 18M6 6l12 12" />
                    </svg>

                    <h2 class="font-[roboto] text-4xl font-semibold text-gray-900 mb-4">
                        {"404 - Page Not Found"}
                    </h2>

                    <p class="font-[roboto] text-gray-900 text-lg mb-6">
                        {"Oops! The page you're looking for doesn't exist."}
                    </p>

                    <button onclick={on_home} class="cursor-pointer font-[oswald] px-6 py-2 bg-gray-500 text-white rounded-lg hover:bg-gray-600 transition">
                        {"Go to Home"}
                    </button>
                </div>
            </Layout>
        </div>
    }

}