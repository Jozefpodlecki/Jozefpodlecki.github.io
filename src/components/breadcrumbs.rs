use yew::*;
use yew_router::prelude::Link;

use crate::{components::icons::HomeIcon, route::Route};

#[function_component(Breadcrumbs)]
pub fn breadcrumbs() -> Html {
   
    html! {
        <nav class="flex" aria-label="Breadcrumb">
            <ol class="inline-flex items-center space-x-1 md:space-x-2 rtl:space-x-reverse">
                <li class="inline-flex items-center">
                <Link<Route>
                    to={Route::Home}
                    classes="inline-flex items-center text-md font-medium text-gray-700 hover:text-blue-600 dark:text-gray-400 dark:hover:text-white">
                    <HomeIcon class="w-3 h-3 me-2.5"/>
                    { "Home" }
                </Link<Route>>
                </li>
                <li>
                <div class="flex items-center">
                    <svg class="rtl:rotate-180 w-3 h-3 text-gray-400 mx-1" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 6 10">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 9 4-4-4-4"/>
                    </svg>
                    <span class="ms-1 text-md font-medium text-gray-700 hover:text-blue-600 md:ms-2 dark:text-gray-400 dark:hover:text-white">
                        {"Projects"}
                    </span >
                </div>
                </li>
            </ol>
        </nav>
    }

}