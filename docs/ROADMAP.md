# Portfolio Engineering Roadmap

Status: feature-complete; release validation and maintenance improvements remain.
Reviewed: 2026-11-03
Basis: `Jozefpodlecki.github.io-responsive-photo-control.zip`, including Rust/Yew source, Trunk configuration, GitHub Actions, documentation, and E2E tests.

## Executive assessment

| Dimension | Rating | Rationale |
| --- | --- | --- |
| Application architecture | 8/10 | Feature-oriented modules, shared services and context, JSON-driven content, clear separation of app and E2E crates. |
| Code maintainability | 7.5/10 | Good domain organization and typed content errors; opportunities to simplify state ownership, avoid repeated content work, and improve consistency. |
| Browser E2E engineering | 8.5/10 | 78 browser tests across 21 test files, reusable harness/page objects, isolated Trunk servers, fixtures, screenshots, and explicit waits. |
| Accessibility and resilience | 7.5/10 | Keyboard and ARIA checks, responsive tests, content fallbacks, and error states; no comprehensive automated accessibility scan in the reviewed suite. |
| CI and release confidence | 7/10 | Production build and Pages deployment exist; automated verification is not currently a prerequisite to deployment. |
| **Overall** | **8/10** | Strong feature-complete portfolio; future effort should prioritize release confidence and maintainability. |

Ratings are qualitative engineering judgments, not measured benchmarks. The archive was inspected statically. Compilation, Chrome E2E execution, Lighthouse, axe, production deployment, and real-device behaviour were **not** verified during this review. An earlier local travel-caption E2E failure was reported and its test was updated, but a fully green subsequent run was not supplied.

## Guiding principles

- Prioritize fixing observable problems over adding speculative features.
- Preserve Rust/Yew, Trunk, the existing `data-component` conventions, feature-oriented modules, JSON content and semantic HTML.
- Prefer deterministic tests of user-visible behaviour over assertions tied to implementation details or incidental prose.
- Keep new features accessible, responsive, theme-aware, and documented from the first change.
- Do not add comments to Rust or UI source merely to explain straightforward code; use documentation for rationale and architecture decisions.
- Avoid complexity that cannot be justified by actual content volume, traffic, or maintenance cost.

## P0 - Verify the current release before more feature work

**Why:** The suite is substantial, but the last reviewed release has not been independently shown to pass in full, and recent test regressions demonstrate that a final gate matters.

- [ ] Run `cargo make verify` and fix all formatting, compilation, Clippy, and unit-test failures.
- [ ] Run `cargo make e2e` from a clean working tree and record a completely green run.
- [ ] Run `cargo make release` and inspect the generated `dist/` assets.
- [ ] Check deployed GitHub Pages, including routing, favicon, CV PDF, images, console errors, and all external links.
- [ ] Review the website at narrow mobile, tablet, desktop, light/dark mode, and both teal/navy palettes.
- [ ] Confirm the `background_photo_identifies_ponta_do_rosto` test remains stable over repeated runs after the caption-wait fix.

**Done when:** Every command passes, the deployed site is manually smoke-tested, and no known failing test remains.

## P1 - Add pre-deployment quality gates

**Finding:** `.github/workflows/deploy.yml` builds with `trunk build --release` and uploads the Pages artifact, but does not run the project's `verify` or E2E tasks before deployment.

- [ ] Add a dedicated CI verification job for Rust formatting, compilation, Clippy, unit tests, and content validation.
- [ ] Make the Pages build/deploy job depend on that verification job.
- [ ] Run a small cross-platform/browser smoke test in CI initially; expand to the full Chrome E2E suite once runtime and flake rates are acceptable.
- [ ] Pin or deliberately manage toolchain/Trunk versions rather than relying on `latest` indefinitely.
- [ ] Keep generated artifacts and failing screenshots available as CI artifacts.

**Done when:** A failing quality check prevents a production deployment and failure artifacts are easy to inspect.

## P1 - Make E2E testing faster and less fragile

**Findings:** The current harness starts a fresh Trunk server and managed WebDriver for each browser test, providing useful isolation but with significant process overhead. Recent issues showed sensitivity to exact alt text and when viewer content becomes readable.

- [ ] Measure total E2E runtime, average startup cost, and which tests are slow/flaky before refactoring.
- [ ] Introduce a clear `smoke` versus `full` suite strategy; execute smoke checks on every pull request and the full suite before release.
- [ ] Consider a shared Trunk server per test binary or fixture, while retaining independent WebDriver sessions and state cleanup. Do not share mutable browser state without isolation.
- [ ] Centralize waits for visible text, dialogs, image readiness, and transition completion in page helpers.
- [ ] Prefer semantic assertions over exact wording unless copy is explicitly part of the requirement; keep accessible labels tested intentionally.
- [ ] Replace hard-coded assumptions tied to the current project count with fixture-based expectations where appropriate.
- [ ] Make palette selection deterministic in screenshot-based runs using the existing `?palette=` override.
- [ ] Audit timeouts and error messages, retaining screenshots and Trunk logs on failure.

**Done when:** Tests pass repeatedly with predictable runtime and failures identify the broken behaviour without frequent timing-only regressions.

## P1 - Strengthen accessibility and visual verification

**Findings:** Current tests check accessible names, ARIA state, navigation, responsive overflow, and modal behaviour. The visual test takes screenshots at multiple widths, but screenshot comparison runs only when `E2E_VISUAL_BASELINE_DIR` is configured.

- [ ] Run an automated accessibility scanner, such as axe-core, against Home, About, Projects, project detail, and both dialogs.
- [ ] Add manual keyboard-only and screen-reader smoke tests for navigation, travel map, photo viewer, pagination, and theme toggle.
- [ ] Verify that focus is trapped when dialogs are open and restored after closing; check Escape and background scrolling.
- [ ] Test reduced-motion mode and sufficient contrast across all four palette/theme combinations.
- [ ] Add committed visual baselines or another intentionally reviewed image-diff workflow for a limited, deterministic set of screens.
- [ ] Include both desktop and small-mobile snapshots, avoiding nondeterministic animation frames.

**Done when:** Automated accessibility checks have no serious findings, keyboard journeys work, and deliberate UI changes require a visual baseline review.

## P2 - Harden content validation and project growth

**Findings:** Project content already has `priority`, slug uniqueness validation, HTTPS checks, sorting, filtering, and nine-per-page pagination. The runtime validator does not yet enforce every content-quality or asset-consistency rule.

- [ ] Add a build-time/content validation command for JSON schemas, duplicate/invalid slugs, missing linked images, required descriptions, and invalid URLs.
- [ ] Validate the profile, education, background, travel and experience data as well as projects.
- [ ] Add a fixture with more than nine projects to the regular verification path, retaining pagination and filter-reset coverage.
- [ ] Decide a stable tie-break policy for equal project priorities and document it clearly.
- [ ] Test behavior when project data changes while the user is viewing a later page.
- [ ] Optionally introduce a small content checklist for each newly published case study: image, summary, tags, links, accessibility and priority.

**Done when:** Incorrect JSON or missing assets are caught before deployment, and publishing a new project is a predictable content-only change.

## P2 - Simplify state and rendering only where profiling supports it

**Opportunities, not proven defects:** `ProjectCollection` clones its project list into presentation properties and builds filtered/visible vectors during rendering. The interactive canvas and pointer effects can be performance-sensitive on lower-powered devices.

- [ ] Profile production WASM startup and interaction timing on a mid-range phone.
- [ ] Measure project-list rendering before optimizing; if it becomes expensive, cache derived filters or reduce cloning with shared references/state.
- [ ] Verify that asynchronous requests and animation effects stop cleanly when components unmount or navigation changes.
- [ ] Profile the flow field in dark/light themes and reduced-motion mode; cap frame work if battery or responsiveness suffers.
- [ ] Review the connection-monitor behaviour for stale overlapping requests and ensure it avoids noisy offline/recovery notifications.
- [ ] Continue applying consistent Rust formatting and small focused function boundaries without fragmenting features into unnecessary modules.

**Done when:** Any refactor has a benchmark, a simpler design, or an identified bug as justification.

## P2 - Measure performance, metadata, and live-site behaviour

- [ ] Run Lighthouse on the actual production origin for mobile and desktop, recording performance, accessibility, best practices and SEO results.
- [ ] Evaluate image size, loading priority, font requests and first-render experience using real measurements.
- [ ] Verify Open Graph/Twitter image previews, canonical URL, sitemap, robots and favicon from the deployed origin.
- [ ] Confirm the published Pages URL and any custom-domain setup match the hard-coded absolute metadata URLs.
- [ ] Revisit hash routing only if independently indexed project pages become a genuine requirement; a router or framework migration is not currently justified.

**Done when:** Baseline numbers are documented and improvements target actual bottlenecks or discoverability problems.

## P3 - Optional future features, only with real demand

- [ ] Preserve pagination page/filter in the URL if visitors need sharable filtered listings.
- [ ] Add search to Projects only when tags and pagination are no longer sufficient.
- [ ] Add richer case-study media or a changelog only when new projects justify them.
- [ ] Consider unobtrusive analytics only if usage data will influence decisions and privacy requirements are addressed.
- [ ] Consider prerendered project detail pages only if search traffic or share-preview requirements justify substantial routing/build changes.

These are deliberately not commitments. Do not pursue them while P0/P1 work remains unfinished.

## Suggested sequence when development resumes

1. **First session:** Pull the latest code, rerun `cargo make verify`, `cargo make e2e`, and `cargo make release`; resolve any failures before touching features.
2. **Next session:** Add CI quality gates, including a deploy-blocking smoke path.
3. **Next session:** Stabilize and speed up browser tests; make visual runs deterministic.
4. **Next session:** Perform axe/Lighthouse and keyboard audits, then prioritize issues by severity.
5. **Later:** Improve content validation and add real projects. Profile and refactor only when measurements support it.

## Definition of done for any future change

- [ ] Behaviour is covered by a unit, integration, or E2E test appropriate to its risk.
- [ ] Works at small mobile and desktop widths and with light/dark and teal/navy themes.
- [ ] Keyboard and accessible-name behaviour is checked for interactive UI.
- [ ] Loading, empty, error, and retry states are considered where relevant.
- [ ] Content/schema, architectural and E2E docs are updated when their contracts change.
- [ ] `cargo make verify`, `cargo make e2e`, and `cargo make release` pass before deployment.
- [ ] Deployment smoke test confirms that the live site behaves as intended.

## What not to rewrite

The current feature modules, data-driven content, Yew/Trunk architecture, domain selectors, shared E2E harness, and established user interactions are all worth preserving. No evidence in the reviewed source warrants a full rewrite or framework migration.
