use yew::prelude::*;

pub(super) const PAGE_SIZE: usize = 9;

#[derive(Properties, PartialEq)]
pub struct ProjectPaginationProps {
    pub current_page: usize,
    pub page_count: usize,
    pub total: usize,
    pub on_change: Callback<usize>,
}

#[function_component(ProjectPagination)]
pub fn project_pagination(props: &ProjectPaginationProps) -> Html {
    if props.page_count <= 1 {
        return Html::default();
    }

    let start = (props.current_page - 1) * PAGE_SIZE + 1;
    let end = (props.current_page * PAGE_SIZE).min(props.total);
    let previous = props.current_page.saturating_sub(1).max(1);
    let next = (props.current_page + 1).min(props.page_count);
    let on_previous = {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit(previous))
    };
    let on_next = {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit(next))
    };

    html! {
        <nav
            data-component="project-pagination"
            aria-label="Project pages"
            class="flex select-none flex-wrap items-center justify-between gap-4 border-t border-divider py-5"
        >
            <p data-component="project-pagination-status" class="m-0 text-sm text-subtle">
                {format!("Showing {start}-{end} of {} projects", props.total)}
            </p>
            <div class="flex flex-wrap items-center gap-2">
                <button
                    type="button"
                    data-component="project-page-previous"
                    aria-label="Previous project page"
                    disabled={props.current_page == 1}
                    onclick={on_previous}
                    class="rounded-lg border border-divider px-3 py-1.5 text-sm text-ink transition-colors hover:border-highlight hover:text-highlight disabled:cursor-not-allowed disabled:opacity-40"
                >
                    {"Previous"}
                </button>
                {for page_items(props.current_page, props.page_count).into_iter().enumerate().map(|(index, item)| {
                    let Some(number) = item else {
                        return html! {
                            <span key={format!("ellipsis-{index}")} aria-hidden="true" class="px-1 text-subtle">{"..."}</span>
                        };
                    };
                    let active = props.current_page == number;
                    let on_change = props.on_change.clone();
                    html! {
                        <button
                            key={number.to_string()}
                            type="button"
                            data-component="project-page-number"
                            data-page={number.to_string()}
                            aria-label={format!("Project page {number}")}
                            aria-current={if active { Some("page") } else { None }}
                            onclick={Callback::from(move |_| on_change.emit(number))}
                            class={classes!(
                                "grid", "min-h-9", "min-w-9", "place-items-center", "rounded-lg",
                                "border", "px-2", "text-sm", "transition-colors",
                                "hover:border-highlight", "focus-visible:outline-2", "focus-visible:outline-highlight",
                                if active { "border-highlight" } else { "border-divider" },
                                if active { "bg-highlight" } else { "bg-card" },
                                if active { "text-on-accent" } else { "text-ink" },
                            )}
                        >
                            {number}
                        </button>
                    }
                })}
                <button
                    type="button"
                    data-component="project-page-next"
                    aria-label="Next project page"
                    disabled={props.current_page == props.page_count}
                    onclick={on_next}
                    class="rounded-lg border border-divider px-3 py-1.5 text-sm text-ink transition-colors hover:border-highlight hover:text-highlight disabled:cursor-not-allowed disabled:opacity-40"
                >
                    {"Next"}
                </button>
            </div>
        </nav>
    }
}

fn page_items(current: usize, total: usize) -> Vec<Option<usize>> {
    let mut pages = vec![1];
    let first = current.saturating_sub(1).max(1);
    let last = (current + 1).min(total);
    for page in first..=last {
        if page != 1 && page != total {
            pages.push(page);
        }
    }
    if total > 1 {
        pages.push(total);
    }
    pages.sort_unstable();
    pages.dedup();

    let mut result = Vec::new();
    let mut previous = 0;
    for page in pages {
        if previous > 0 && page > previous + 1 {
            result.push(None);
        }
        result.push(Some(page));
        previous = page;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pages_are_bounded_for_large_collections() {
        assert_eq!(page_items(6, 20), vec![Some(1), None, Some(5), Some(6), Some(7), None, Some(20)]);
    }

    #[test]
    fn small_collections_show_all_page_numbers() {
        assert_eq!(page_items(2, 3), vec![Some(1), Some(2), Some(3)]);
    }
}
