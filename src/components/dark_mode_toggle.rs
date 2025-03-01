use yew::prelude::*;
use web_sys::window;

#[function_component(DarkModeToggle)]
pub fn dark_mode_toggle() -> Html {
    let is_dark_mode = use_state(|| {
        window()
            .and_then(|w| w.document())
            .map(|doc| doc.document_element())
            .flatten()
            .map_or(false, |root| root.class_list().contains("dark"))
    });

    let toggle_dark_mode = {
        let is_dark_mode = is_dark_mode.clone();
        Callback::from(move |_| {
            if let Some(document) = window().and_then(|w| w.document()) {
                if let Some(root) = document.document_element() {
                    let class_list = root.class_list();

                    if *is_dark_mode {
                        class_list.remove_1("dark").unwrap_or_else(|_| {});
                    } else {
                        class_list.add_1("dark").unwrap_or_else(|_| {});
                    }
                }
            }

            is_dark_mode.set(!*is_dark_mode);
        })
    };

    html! {
        <div class="flex justify-end transition-colors">

            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="mr-2"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="white"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round">
                <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                <path d="M12 12m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" />
                <path d="M3 12h1m8 -9v1m8 8h1m-9 8v1m-6.4 -15.4l.7 .7m12.1 -.7l-.7 .7m0 11.4l.7 .7m-12.1 -.7l-.7 .7" />
            </svg>

            <label data-dark-mode-toggle="true" class="relative inline-block w-[35px]">
                <input class="peer opacity-0 w-0 h-0" type="checkbox"/>
                <span onclick={toggle_dark_mode} class="slider duration-300 h-[20px] absolute top-0 left-0 right-0 bottom-0 cursor-pointer bg-white before:absolute before:content-[''] before:w-[20px] before:h-[20px] before:bg-black before:left-0 before:right-0 before:duration-300 peer-checked:bg-black peer-checked:before:bg-white"></span>
            </label>

            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="mx-2"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="white"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="">
                <path d="M12 3c.132 0 .263 0 .393 0a7.5 7.5 0 0 0 7.92 12.446a9 9 0 1 1 -8.313 -12.454z" />
            </svg>
        </div>
    }
}
