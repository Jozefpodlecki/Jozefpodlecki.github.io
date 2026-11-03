# Engineering conventions

- Group functionality into cohesive feature modules; avoid unrelated helpers and one-file-per-small-component fragmentation.
- Use readable multiline Yew markup and meaningful component boundaries.
- Do not write Rust, Yew, CSS or configuration comments. Explain implementation choices in Markdown.
- Prefer Tailwind v4 utilities for layout and interactive styling. Keep global theme tokens, wallpaper composition and the Canvas visual overlays in `styles/main.css`.
- Use accessible HTML, named icon controls, visible focus and reduced-motion support.
- Mark substantial components with `data-component` and record identities with attributes such as `data-project-slug`, `data-filter-tag` and `data-social`. Do not introduce `data-testid`.
- Keep project order in `projects.json` via ascending `priority`, with equal priorities retaining source order. Keep gallery pagination in the projects feature; featured cards remain limited to three.
- Use the existing `AppEnvironment` and state abstractions instead of new browser globals or unnecessary external state managers.
- Keep JSON content typed and public content free of credentials and private contact details.
- Preserve Trunk-managed Tailwind and wasm-bindgen; do not introduce npm or Node-based hooks.
- Use Thirtyfour managed WebDriver sessions, isolated Axum fixture state and typed error-returning E2E assertions.
- E2E browser tests must return errors instead of panicking so screenshots, tracing and asynchronous WebDriver cleanup can run.
- Do not claim checks passed unless they ran. On toolchain-limited machines, report the validation gap.

See `docs/DEVELOPMENT.md`, `docs/ARCHITECTURE.md`, `docs/COMPONENTS.md`, and `docs/E2E.md`.
