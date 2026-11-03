# Yew component conventions

Organise components by cohesive feature rather than creating one file for every small component. Keep HTML/Yew attributes readable across lines and prefer reusable components for meaningful page sections, shared state or distinct lifecycles.

## Markup and selectors

Substantial component roots use `data-component="kebab-case-name"`. Collection records can expose `data-project-slug`, `data-country`, `data-filter-tag` or another descriptive identifier. E2E tests should use those semantic contracts and standard accessible attributes, not Tailwind classes or `data-testid`.

Prefer native `<button>`, `<a>` and semantic heading elements. Use `aria-label` for icon-only controls and visible labels whenever available. Provide `type="button"` on buttons not intended to submit forms. Navigation and action icons use `yew_icons`, without raw arrow glyphs.

## Components by feature

| Module | Main responsibilities |
| --- | --- |
| `layout/shell.rs` | BrowserShell, background, theme, route presentation and global context-menu policy |
| `layout/navigation.rs` | Route links and active navigation state |
| `layout/photo/` | Camera trigger, fullscreen photograph dialog, fit controls, Escape and click-away dismissal |
| `layout/footer.rs` | SiteFooter and technology credits |
| `portfolio.rs` | Home, About, Hero, SocialLinks and FeaturedProjects |
| `hero/mod.rs` | HeroFlowField, Canvas lifecycle and pointer callbacks |
| `hero/field.rs` | Flow-field computation and Canvas renderer |
| `experience.rs` | Work experience timeline and entries |
| `education/` | Education entries on the About page |
| `projects/state.rs` | Shared project loading, validation and retry context |
| `projects/cards.rs` | Project cards and external actions |
| `projects/collection.rs` | Filtering, ordering-aware results and pagination state |
| `projects/pagination.rs` | Accessible previous/next and numbered project pages |
| `projects/detail.rs` | Case study, heading, cover, narrative and actions |
| `projects/artwork/` | Shared image loading and accessible fallback |
| `travel/` | Map rendering, visited-country tooltips and fullscreen dialog |
| `recovery.rs` | ContentRecovery for fetch/validation errors |
| `motion.rs` | Animato MotionReveal |

## Layout and motion

Tailwind v4 handles component layout, spacing, typography and most hover/focus states. The single Tailwind entry point is `styles/tailwind.css`; the imported `styles/main.css` supplies shared tokens and specialist CSS. The Canvas hero has distinct light and dark palettes and obeys reduced motion. It does not intercept the site's primary interactive controls or replace meaningful content.

Project artwork is sourced from runtime JSON, with a deliberate fallback rather than a broken `<img>` when a file cannot be decoded. The About portrait similarly offers a graceful fallback. Do not claim conceptual illustrations are screenshots of running software.

## Keyboard and interaction

The route shell has a skip link targeting the focusable main content area. The map modal manages focus entry, keyboard cycling, Escape and focus restoration. Visited countries support keyboard activation and unvisited countries remain noninteractive. The fullscreen photograph viewer keeps the underlying route mounted, manages focus, disables background scrolling, and closes with Back, Escape or an outside click. Its fit control switches between full-viewport cropping and the entire image.

The browser context menu is available in debug builds and disabled at the application shell in release builds by design. It is not a security boundary. Non-selectable presentation headings, buttons and experience cards are separate from the selection behaviour of longer case-study text.

## State and errors

Use shared `ProjectsContext` for project data across Home, gallery and detail routes. Use `Load<T>` and typed `ContentError` for fetch, validation and retry flows. Avoid adding global state for local UI interactions or passing stringly typed errors through components.

`hero/info.rs` provides a floating, anchored flow-field explanation popover with click-away and Escape dismissal, plus Calm/Dynamic controls.

## Connection status

`features/layout/connection.rs` owns the non-blocking `ConnectionNotice`. It subscribes to browser online/offline events and uses a localhost-only HTTP probe to identify a stopped development server. It does not intercept unrelated JavaScript exceptions or replace content-specific error recovery.
