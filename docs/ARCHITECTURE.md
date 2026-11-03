# Architecture

## Runtime and rendering

Trunk builds `crates/web/` into a client-side Yew 0.23 application targeting WebAssembly. `index.html` renders a CSS-only pre-mount splash. `main.rs` initializes `AppEnvironment`, browser logging and the Yew application. `app.rs` loads essential profile and experience content and controls startup recovery. The router uses hash URLs so browser navigation works on static hosting.

## Content and state

`content.rs` defines Serde models, runtime fetches, validation and `ContentError` variants for HTTP, network, decoding and invalid content. `state.rs` represents loading, ready and error states with `Load<T>`. Essential profile data is shared through Yew Context. `features/projects/state.rs` supplies a single project load/retry context to the featured collection, gallery and case-study routes. Travel content has its own fetch and retry lifecycle; it does not block other pages.

All JSON under `crates/web/public/data/` is public. It is copied to `dist/data/` and requested over HTTP when needed. `world_countries.json` is compiled into the WASM application as local SVG map geometry.

## Feature boundaries

| Feature | Responsibility |
| --- | --- |
| `features/layout/` | Browser shell, navigation, theme toggle, background, fullscreen photo viewer and footer |
| `features/portfolio.rs` | Homepage hero, featured projects and About layout |
| `features/experience.rs` | Reusable work-history timeline and entries |
| `features/education/` | Profile-driven education section on About |
| `features/projects/` | Shared loading state, stable project-priority sorting, filters, nine-card pagination, artwork fallback and case studies |
| `features/travel/` | Visit data, interactive world map and fullscreen dialog |
| `features/hero/` | Canvas 2D vector field, particles and animation lifecycle |
| `features/motion.rs` | Animato-driven reveal transitions |
| `features/recovery.rs` | Human-readable recovery UI for failed content requests |

`env.rs` provides platform access for window, theme and motion timing. Browser-dependent state remains in the appropriate Yew hook or feature rather than in a new global state manager.

## Styling and motion

Tailwind CSS v4 is compiled by Trunk from `styles/tailwind.css`. It imports `styles/main.css`, which holds shared theme tokens, typography, background visuals, custom scrollbars and specialist effects. `MotionReveal` uses Animato 1.7.3 for fades. `HeroFlowField` uses Canvas 2D with `requestAnimationFrame`, a capped frame rate, pointer influence and a static reduced-motion frame. The canvas is decorative and hidden on narrow layouts.

## Testing

`crates/e2e/` uses Thirtyfour with managed ChromeDriver and a Trunk subprocess per normal browser test. Axum serves isolated fixtures from `dist/`, allowing missing, malformed and delayed JSON responses to be tested. Typed test assertion errors allow the harness to capture screenshots and close WebDriver normally. `tracing` records named test spans and subprocess diagnostics.

For deeper implementation details see [MODULES.md](MODULES.md), [MOTION.md](MOTION.md) and [E2E.md](E2E.md).
