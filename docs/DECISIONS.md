# Architecture decisions

| Decision | Rationale |
| --- | --- |
| Rust, Yew CSR and Trunk | Single Rust browser application with reproducible WASM build tools |
| HashRouter | Works with static GitHub Pages hosting without rewrite rules |
| Public runtime JSON | Content edits can be deployed without rewriting Yew components |
| Yew Context for shared state | Profile and project data are shared without an additional state-management framework |
| Typed content failures | Loading errors provide an actionable retry instead of a raw exception |
| Tailwind v4 and a small CSS layer | Component layouts stay close to markup; shared tokens and specialty effects stay in CSS |
| Animato and Canvas 2D | Animato handles reveal values; Canvas handles a contained technical hero visual |
| Accessibility-first motion | Reduced-motion preference disables movement without withholding content |
| Bundled project artwork | Predictable local assets, with a visible fallback if loading fails |
| Local SVG world map | No third-party tile service or location tracking |
| Rust E2E with Thirtyfour and Axum | Real browser journeys plus deterministic failure fixtures |
| Global context-menu suppression | Requested presentation preference; it does not secure public content |

The repository's root `CNAME` is not copied by the current GitHub Pages workflow. If a custom domain is configured later, manage it explicitly in GitHub Pages settings and revisit publishing requirements.
