use yew::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub full_height_and_center: bool,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {

    if props.full_height_and_center {
        return html! {
            <div class="flex-col-center justify-between h-screen bg-gradient-to-t from-black via-gray-900 to-gray-800">
                <div class="flex justify-center bg-white dark:bg-black max-w-[800px] w-full p-8 rounded-lg shadow-lg m-8 flex flex-col justify-between h-full border dark:border-white">
                    { for props.children.iter() }
                </div>
            </div>
        }
    }

    html! {
        <div class="flex-col-center justify-between min-h-screen bg-gradient-to-t from-black via-gray-900 to-gray-800">
            <div class="bg-white dark:bg-black max-w-[800px] w-full p-8 rounded-lg shadow-lg m-8 flex flex-col justify-between h-full border dark:border-white">
                { for props.children.iter() }
            </div>
        </div>
    }
}