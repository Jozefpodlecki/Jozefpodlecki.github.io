use yew::*;

use crate::components::{dark_mode_toggle::DarkModeToggle, layout::Layout};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub reload: Callback<()>,
}

#[function_component(ErrorPage)]
pub fn error(props: &Props) -> Html {    
    let on_reload = {
        let reload = props.reload.clone();
        Callback::from(move |_| reload.emit(()))
    };

    html! {
        <>
            <div class="h-0 sticky top-1 right-1">
                <DarkModeToggle/>
            </div>
            <div>
                <Layout full_height_and_center={true}>
                    <div class="flex-col-center">
                        <svg class="w-12 h-12 text-red-600 mb-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
                            <path d="M10.29 3.86L1.82 18a1 1 0 00.9 1.5h18.56a1 1 0 00.9-1.5L13.71 3.86a1 1 0 00-1.72 0z"></path>
                            <line x1="12" y1="9" x2="12" y2="13"></line>
                            <line x1="12" y1="17" x2="12.01" y2="17"></line>
                        </svg>
                        <h2 class="font-[roboto] text-2xl font-bold text-gray-900 dark:text-white mb-2">
                            {"Something Went Wrong"}
                        </h2>
                        <p class="font-[roboto] text-gray-700 dark:text-gray-300 mb-4 text-center">
                            {"We couldn't load the data. Please try again later or contact support."}
                        </p>
                        <button onclick={on_reload} class="font-[oswald] cursor-pointer px-4 py-2 bg-gray-500 text-white rounded-lg hover:bg-gray-600 transition">
                            {"Reload Page"}
                        </button>
                    </div>
                </Layout>
            </div>
        </>
    }
}