# Design system

The portfolio is a restrained, editorial interface that balances a professional software-engineering profile with personal photography and an interactive technical visual.

## Layout and hierarchy

The navigation exposes Home, About and Projects, with the theme toggle at the end. The homepage hero uses a two-column layout above the mobile breakpoint: a large two-line introduction on the left and a procedural flow-field canvas on the right. The Madeira photograph remains in the shared background and a fullscreen photography viewer opened by the camera button; it is not the hero panel image. The canvas is hidden on narrow screens so the introduction remains readable.

Featured projects use a responsive card grid with meaningful local illustrations and internal case-study links. About uses a biography, portrait, work timeline and interactive map. The project detail page uses a header, wide artwork and a centered reading column without a duplicate technology sidebar.

## Typography and colour

- Space Grotesk for display headings and the JP mark; Manrope for body text and controls.
- Light and dark theme tokens are defined in `styles/main.css`.
- Teal is the accent for active navigation, links and visited countries.
- Use readable paragraph widths and visible keyboard focus indicators.
- Preserve contrast when changing theme-specific visual effects.

## Hero flow field

`features/hero/field.rs` draws a newly seeded vector field with streamlines, particles and two gentle vortices. `features/hero/info.rs` provides a keyboard-accessible anchored explanation popover above its trigger without shifting the hero layout. `features/hero/mod.rs` owns the Yew component, pointer events, Canvas setup and animation frame lifecycle. The light palette uses a pale surface, darker teal lines and minimal shadowing; the dark palette uses luminous teal trails and a deep vignette. A media preference for reduced motion creates a static frame, and the field is decorative for assistive technology.

## Themes, effects and CSS ownership

Tailwind v4 classes in Yew components own layout, spacing, responsive behaviour and most interaction states. `styles/main.css` owns global tokens, Madeira wallpaper layering, the dark-only cursor glow, custom scrollbar styling and the theme-specific Canvas overlays. `index.html` owns critical splash CSS because it must render before WebAssembly boots. Do not add an npm/Tailwind build pipeline.

The photograph viewer uses the original unfiltered Madeira image, a dark photographic stage, an unobtrusive location caption, and a reversible Fill screen / Entire photo option. The portfolio remains mounted behind it and the viewer closes without changing routes.

The light theme intentionally has no trailing cursor glow. The dark theme glow is subtle and enabled only on fine-pointer, hover-capable devices when reduced motion is not requested. Do not hide the native cursor.

## Project imagery

Sudoku, win-drives and Zipeek have bundled WebP illustrations. These are conceptual artwork, not screenshots of the running apps. A shared `ProjectArtwork` component reserves the media space and presents a themed accessible fallback when an image is absent or fails to load.

## Accessibility and interaction

Interactive controls expose descriptive names and keyboard focus. The fullscreen travel-map dialog traps focus, closes with Escape and restores focus to the expand button. The skip-navigation link focuses main content without changing the hash route. Text selection is disabled on presentation-only areas, while essential long-form content remains readable. The global context-menu restriction is a UI preference, not asset protection.

## Colour palettes

The site supports Teal and Navy colour palettes, each with a light and dark variant. The palette is chosen randomly once at document load, before the splash screen renders. Light and dark mode remain a separate persistent preference. For deterministic reviews or E2E tests, use `?palette=teal` or `?palette=navy` before the hash route, such as `/?palette=navy#/projects`. The visual palette is not stored across refreshes. All primary colours are CSS design tokens consumed by Tailwind utilities, the map and the Canvas flow field.
