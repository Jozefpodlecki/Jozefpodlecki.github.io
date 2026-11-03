# Module boundaries

Features are grouped by responsibility rather than one file per component. Extract a new module when an independent concern needs its own state, lifecycle or rendering logic.

```text
crates/web/src/
  app.rs                  Startup and essential content
  content.rs              Models, fetch and validation
  env.rs                  Browser, theme and motion services
  routes.rs               Hash routes
  state.rs                Load<T>
  features/
    portfolio.rs          Home and About page composition
    experience.rs         Work timeline
    hero/                 Canvas flow field and lifecycle
    layout/               Shell, navigation, footer, photo UI
    motion.rs             Animato reveal
    projects/             State, cards, collection, pagination, detail, artwork
    travel/               SVG map and visited-country interactions
    recovery.rs           Content recovery
crates/e2e/
  src/                    Harness, fixtures, assertions and helpers
  tests/                  Browser journeys and asset checks
```

## Coding conventions

Keep Yew markup readable across lines, avoid unnecessary fragments and prefer named components for meaningful sections. Use `data-component` and domain attributes for stable E2E selectors; never introduce `data-testid`. Use `yew_icons` for interface icons. Do not add comments to Rust, Yew, CSS or configuration files. Place rationale in these Markdown docs instead.

`styles/tailwind.css` is the single Trunk-managed Tailwind entry point. Use Tailwind utilities in markup and `styles/main.css` for theme tokens, global typography, scrollbar styles, Madeira background and the light/dark Canvas overlays.

## Browser and test boundaries

Browser access starts with `AppEnvironment` and its services. Shared fetched project state is in `features/projects/state.rs`, not duplicated per route. `features/hero/mod.rs` owns the Canvas animation lifecycle while `field.rs` owns its math and drawing. Trunk, Thirtyfour and Axum lifecycles belong to `crates/e2e`, not the web application.

See [ARCHITECTURE.md](ARCHITECTURE.md), [COMPONENTS.md](COMPONENTS.md) and [E2E.md](E2E.md).
