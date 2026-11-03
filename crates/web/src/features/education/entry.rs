use crate::content::Education;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq)]
pub(super) struct EducationEntryProps {
    pub item: Education,
}

#[function_component(EducationEntry)]
pub(super) fn education_entry(props: &EducationEntryProps) -> Html {
    let logo_failed = use_state(|| false);

    {
        let logo_failed = logo_failed.clone();
        use_effect_with(props.item.logo.clone(), move |_| {
            logo_failed.set(false);
            || ()
        });
    }

    let on_logo_error = {
        let logo_failed = logo_failed.clone();
        let institution = props.item.institution.clone();
        Callback::from(move |_: Event| {
            log::warn!("Education logo unavailable for {institution}");
            logo_failed.set(true);
        })
    };

    let show_logo = !props.item.logo.trim().is_empty() && !*logo_failed;

    html! {
        <article
            data-component="education-entry"
            class="flex flex-col gap-3 rounded-xl border border-divider bg-card/60 p-5 backdrop-blur-md sm:flex-row sm:items-start sm:justify-between sm:gap-6 sm:p-6"
        >
            <div class="flex min-w-0 items-start gap-4">
                <div
                    data-component="education-logo"
                    data-image-status={if show_logo { "image" } else { "fallback" }}
                    class="grid size-14 shrink-0 place-items-center overflow-hidden rounded-lg bg-layer sm:size-16"
                >
                    if show_logo {
                        <img
                            src={props.item.logo.clone()}
                            alt={format!("{} logo", props.item.institution)}
                            loading="eager"
                            draggable="false"
                            onerror={on_logo_error}
                            class="h-full w-full object-contain"
                        />
                    } else {
                        <span
                            data-component="education-logo-fallback"
                            role="img"
                            aria-label={format!("Education at {}", props.item.institution)}
                            class="grid h-full w-full place-items-center rounded-lg border border-divider bg-card/70 text-highlight"
                        >
                            <Icon
                                data={IconData::LUCIDE_GRADUATION_CAP}
                                width="28"
                                height="28"
                            />
                        </span>
                    }
                </div>
                <div class="min-w-0 space-y-1">
                    <h3 class="!m-0 text-lg font-semibold text-ink sm:text-xl">
                        {&props.item.qualification}
                    </h3>
                    if props.item.website.is_empty() {
                        <p class="!m-0 text-sm font-medium text-highlight sm:text-base">
                            {&props.item.institution}
                        </p>
                    } else {
                        <a
                            data-component="education-institution-link"
                            href={props.item.website.clone()}
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center gap-1.5 text-sm font-medium text-highlight underline-offset-4 hover:underline focus-visible:outline-2 focus-visible:outline-highlight sm:text-base"
                        >
                            {&props.item.institution}
                            <Icon
                                data={IconData::LUCIDE_ARROW_UP_RIGHT}
                                width="15"
                                height="15"
                            />
                        </a>
                    }
                    <p class="!m-0 text-sm text-subtle">
                        {&props.item.location}
                    </p>
                </div>
            </div>
            <span class="shrink-0 select-none text-sm font-semibold text-highlight sm:text-right">
                {&props.item.period}
            </span>
        </article>
    }
}
