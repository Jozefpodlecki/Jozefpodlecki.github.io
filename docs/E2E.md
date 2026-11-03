# E2E testing

`crates/e2e/` contains the Rust browser automation harness, isolated Axum HTTP fixtures, screenshot utilities and integration scenarios. Thirtyfour 0.38 uses the managed ChromeDriver feature and Tokio runs the asynchronous tests.

## Requirements and quick start

Install Rust, Trunk and Chrome or Chromium. The first managed driver launch may download matching driver files. From the repository root:

```powershell
cargo test -p portfolio-e2e --tests --no-run
cargo make e2e
```

`cargo make e2e` first builds the site into `dist/` for fixture tests, then runs the integration suite with one test thread. Normal browser tests start and stop their own Trunk server on an ephemeral port. Fixture tests serve the built distribution through Axum, using separate state and ports.

For a focused run:

```powershell
cargo test -p portfolio-e2e --test journeys -- --nocapture
cargo test -p portfolio-e2e --test hero_flow -- --nocapture
cargo test -p portfolio-e2e --test assets -- --nocapture
cargo test -p portfolio-e2e --test pagination -- --nocapture
```

## What is covered

- Home, About, Projects, case studies, routing, navigation and conditional external links
- Project filters, nine-per-page navigation (23-project fixture), ordering, loaded artwork and missing-image fallback
- Essential content loading, missing/malformed content and retry recovery
- Travel-map tooltip interactions, modal open/close and keyboard focus handling
- Theme persistence, reduced-motion preference and the Canvas hero
- Responsive navigation, image loading and horizontal overflow checks
- Fullscreen photo viewer, photo button visibility below and above 640px, image fit modes, dismissal, focus restoration, route preservation, selection and context-menu rules
- Browser icons, social links, public content and production distribution assets

The Axum fixture scenarios include successful data, missing resources, malformed or invalid JSON, delayed responses, a synthetic 23-project gallery and recoverable HTTP failures. The suite also tests repeated retries before a success response. Browser-free fixture and assertion tests run with:

```powershell
cargo make unit
```

## Test architecture

The shared harness owns Trunk lifecycle, managed ChromeDriver and failure capture. Reusable page objects and selectors live in `src/pages/` and the harness modules. Use stable `data-component` and domain attributes rather than querying Tailwind class names. Avoid hard-coded sleeps; use element readiness and `wait_for_motion()` for animated content.

Inside asynchronous browser actions, use `portfolio_e2e::ensure!`, `ensure_eq!` and `ensure_ne!` instead of panicking assertions. Their typed failures allow tracing, screenshot capture, location diagnostics and `WebDriver::quit().await` to complete. Browser-free Rust unit tests can use ordinary assertions.

## Logs and screenshots

Failures capture a screenshot, when possible, under `target/e2e-artifacts/`. Trunk stdout and stderr are captured in per-test `trunk-*.log` files. The harness emits named `tracing` spans; to inspect more detail:

```powershell
$env:RUST_LOG = "portfolio_e2e=debug,thirtyfour=warn"
cargo make e2e
```

`E2E_HEADLESS` defaults to true; `E2E_TIMEOUT_SECS` defaults to 20. `E2E_ARTIFACTS_DIR` overrides the artifact directory.

## Visual baselines

The `visual` test captures browser screenshots. Baseline comparison is **optional**, not automatic: approve reference PNGs in a separate directory and set `E2E_VISUAL_BASELINE_DIR` before running the test. `E2E_VISUAL_MAX_MAE` controls the allowed mean channel difference (default `5.0`). Do not treat a screenshot capture as proof that a visual comparison passed.

```powershell
cargo make screenshots
$env:E2E_VISUAL_BASELINE_DIR = "C:\path\to\approved-baselines"
cargo test -p portfolio-e2e --test visual -- --nocapture
```

## Deployed-site smoke test

A local production-asset test verifies `dist/` files and WebP decoding, not the CDN. An opt-in live-site test checks a deployed origin when configured:

```powershell
$env:E2E_DEPLOYED_URL = "https://jozefpodlecki.github.io"
cargo test -p portfolio-e2e --test assets -- --nocapture
cargo test -p portfolio-e2e --test pagination -- --nocapture
```

Review [QA.md](QA.md) for manual checks not fully captured in browser automation.

The flow-field checks also verify randomized initialization and the dismissible explanation. The travel suite compares embedded and expanded tooltip text for multi-visit, origin, and airport-transit records. The hero flow suite also checks Calm/Dynamic switching without resetting the seed.

## Palette and connection tests

`palette_connection.rs` verifies that a URL-selected Navy palette uses the correct accent colour and remains stable through theme changes and route navigation. It also exercises the browser offline notice and dismissal. For local visual testing, open `/?palette=navy#/` or `/?palette=teal#/`. A genuine Trunk shutdown is best checked manually: open the site, stop `trunk serve`, and allow approximately seven seconds for the probe to report disconnection.
