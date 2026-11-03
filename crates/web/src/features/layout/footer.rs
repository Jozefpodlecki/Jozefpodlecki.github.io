use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[function_component(SiteFooter)]
pub fn site_footer() -> Html {
    html! {
        <footer
            data-component="site-footer"
            class="flex flex-col items-start gap-3 border-t border-divider py-8 text-sm text-subtle sm:flex-row sm:items-center sm:justify-between"
        >
            <div
                data-component="technology-credit"
                class="flex select-none flex-wrap items-center gap-2"
            >
                <span>{"Built with"}</span>
                <a
                    data-component="technology-link"
                    data-technology="rust"
                    href="https://www.rust-lang.org/"
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="Rust programming language"
                    class="inline-flex items-center gap-1.5 font-medium text-ink no-underline transition-colors hover:text-highlight focus-visible:rounded-sm focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-highlight"
                >
                    <Icon data={IconData::SIMPLE_ICONS_RUST} width="20" height="20" />
                    <span>{"Rust"}</span>
                </a>
                <span>{"&"}</span>
                <a
                    data-component="technology-link"
                    data-technology="yew"
                    href="https://yew.rs/"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="font-medium text-ink no-underline transition-colors hover:text-highlight focus-visible:rounded-sm focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-highlight"
                >
                    {"Yew"}
                </a>
            </div>
            <div class="flex flex-wrap items-center gap-3 sm:gap-4">
                <a
                    data-component="portfolio-source-link"
                    href="https://github.com/Jozefpodlecki/Jozefpodlecki.github.io"
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="Portfolio source code on GitHub"
                    class="inline-flex select-none items-center gap-1.5 font-medium text-subtle no-underline transition-colors hover:text-highlight focus-visible:outline-2 focus-visible:outline-highlight"
                >
                    <Icon data={IconData::SIMPLE_ICONS_GITHUB} width="17" height="17" />
                    <span>{"Source"}</span>
                </a>
                <span data-component="copyright" class="select-none">{"© 2026 Józef Podlecki"}</span>
            </div>
        </footer>
    }
}
