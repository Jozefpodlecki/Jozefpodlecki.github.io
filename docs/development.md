# Development

## Requirements

- Rust and Cargo, plus the `wasm32-unknown-unknown` target
- Trunk and cargo-make on `PATH`
- Chrome or Chromium for browser E2E tests
- A connection for the initial tool/driver downloads

```powershell
rustup target add wasm32-unknown-unknown
cargo install trunk cargo-make
```

Use the repository root for the `cargo make` commands below. `crates/web/Trunk.toml` configures Tailwind CSS 4.3.3 and wasm-bindgen 0.2.130 as Trunk-managed tools. There is no npm-based build step.

## Run and build

```powershell
cargo make dev
cargo make build
cargo make release
```

The development server is at <http://127.0.0.1:1420/>. Trunk runs from `crates/web/` and outputs to the repository-root `dist/` directory. Open the site through HTTP; loading `dist/index.html` directly with `file://` does not provide the HTTP environment needed for WASM loading and runtime JSON requests. The pre-mount splash has its own inline CSS to avoid an unstyled first paint.

## Check the workspace

```powershell
cargo make fmt-check
cargo make check
cargo make clippy
cargo make unit
cargo make verify
```

`cargo make verify` combines format validation, workspace checks, Clippy and browser-free E2E infrastructure tests. It does not start Chrome. For browser tests:

```powershell
cargo test -p portfolio-e2e --tests --no-run
cargo make e2e
```

`cargo make e2e` builds `dist/` for Axum fixture tests, then runs the browser suite serially. Ordinary browser tests launch a managed Trunk server and managed ChromeDriver. Screenshots and per-test Trunk logs go to `target/e2e-artifacts/`. See [E2E.md](E2E.md) for flags, tracing and visual baselines.

## Editing the site

- Change biography, work history, projects, travel notes or photographic metadata in `crates/web/public/data/*.json`.
- Keep project artwork in `crates/web/public/images/` and update the `image` property of each project.
- Update the public CV in `crates/web/public/` and its `copy-file` declaration and URL together when renaming the PDF.
- Use Tailwind classes in Yew markup for layout and interaction. Keep global theme tokens, scrollbar rules and specialist effects in `styles/main.css`.
- Keep `data-component` and domain-specific attributes stable when changing component markup; the E2E suite relies on them.

## Troubleshooting

| Symptom | Check |
| --- | --- |
| CSS utilities are missing | `index.html` Tailwind link, `styles/tailwind.css`, and Trunk output |
| JSON is missing or invalid | DevTools Network requests to `data/*.json` and the content recovery UI |
| Project artwork fails | `dist/images/`, `public/images/`, and the project `image` path |
| Browser E2E cannot connect | `target/e2e-artifacts/trunk-*.log`, Trunk executable and local firewall |
| ChromeDriver cannot start | Installed Chrome/Chromium and managed-driver provisioning |
| A screenshot test fails | Captured image, theme, viewport and approved baseline; do not blindly update baselines |
| A CSS change works in dark mode only | Check light-mode tokens and theme-specific effects, especially the Canvas hero |

## Release preparation

Run `cargo make verify`, `cargo make e2e` and `cargo make release`; then inspect both themes and a narrow mobile viewport. Confirm the compiled `dist/` includes JSON, project WebP files, favicons and your reviewed public PDF. See [QA.md](QA.md) and [DEPLOYMENT.md](DEPLOYMENT.md).

## Context menu behaviour

The root shell allows the standard browser context menu in debug builds (`trunk serve` by default). Release builds compiled with `--release` suppress the context menu on the portfolio shell. This is a presentation preference, not content protection. The E2E suite runs a debug Trunk server and expects the menu to remain available.
