use crate::{state::Load, content::{fetch_json, sort_projects, validate_projects, Project}};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ProjectsContext {
    pub state: Load<Vec<Project>>,
    pub retry: Callback<MouseEvent>,
}

#[derive(Properties, PartialEq)]
pub struct ProjectsProviderProps {
    pub children: Children,
}

#[function_component(ProjectsProvider)]
pub fn projects_provider(props: &ProjectsProviderProps) -> Html {
    let state = use_state(|| Load::<Vec<Project>>::Loading);
    let attempt = use_state(|| 0_u32);

    {
        let state = state.clone();
        use_effect_with(*attempt, move |_| {
            state.set(Load::Loading);
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_json::<Vec<Project>>("projects.json").await.and_then(|items| {
                    validate_projects(&items)?;
                    let mut items = items;
                    sort_projects(&mut items);
                    Ok(items)
                }) {
                    Ok(items) => state.set(Load::Ready(items)),
                    Err(error) => {
                        log::warn!("Projects load failed: {error}");
                        state.set(Load::Error(error));
                    }
                }
            });
            || ()
        });
    }

    let retry = {
        let attempt = attempt.clone();
        Callback::from(move |_| attempt.set((*attempt).wrapping_add(1)))
    };

    html! {
        <ContextProvider<ProjectsContext> context={ProjectsContext {
            state: (*state).clone(),
            retry,
        }}>
            {for props.children.iter()}
        </ContextProvider<ProjectsContext>>
    }
}
