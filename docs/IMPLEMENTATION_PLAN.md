# Release readiness

This document is a checklist, not a claim that a release has already passed every step.

## Current functionality

The repository includes the Rust/Yew CSR portfolio, Trunk-managed Tailwind v4, a critical-CSS splash, real case studies for Sudoku, win-drives and Zipeek, a travel map, a Canvas 2D flow-field hero, Animato reveals, typed content recovery and a browser E2E harness with Axum fixtures. Reduced motion, keyboard navigation and image-fallback behaviour have automated checks.

## Before publishing

- [ ] `cargo make verify` passes on the final working tree.
- [ ] `cargo make e2e` passes on the final working tree.
- [ ] `cargo make release` produces `dist/` with public JSON, images, icons and CV.
- [ ] Inspect the light and dark flow-field panels, desktop and mobile views.
- [ ] Check the published PDF contains only information intended for public access.
- [ ] Review GitHub Pages workflow results and the live site.
- [ ] Verify keyboard skip-link, fullscreen map Escape/focus restoration and reduced-motion display.

## Optional future improvements

- Replace conceptual project artwork with authentic screenshots.
- Approve reference screenshots for opt-in visual regression comparisons.
- Add more structured case-study material when the underlying project repositories support it.
- Revisit global context-menu suppression if it conflicts with user expectations.

See [QA.md](QA.md) and [DEPLOYMENT.md](DEPLOYMENT.md).
