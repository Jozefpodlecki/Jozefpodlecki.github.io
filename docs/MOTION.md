# Motion and Canvas rendering

## Animato reveals

The web crate uses `animato = "1.7.3"`. `features/motion.rs` uses `Tween` and easing values to fade and slightly translate content into view. `MotionService` in `env.rs` reads the browser's `Performance.now()` timing and reduced-motion preference; the service is supplied through Yew Context. Do not use Rust's native `Instant` in the WASM browser animation loop.

`MotionReveal` manages its timer and unmount cleanup. Essential navigation and recovery UI should never depend on completing an animation. Screenshot tests wait until visible reveals settle.

## Flow-field hero

`features/hero/field.rs` computes a seeded normalized vector field with two vortices, draws streamlines and advances particles. `features/hero/info.rs` owns the accessible "What is this?" disclosure below the canvas. Each canvas mount generates a new seed for particle positions and field phase; `features/hero/mod.rs` owns the canvas element, pointer state, animation-frame handle and resize behaviour. Drawing follows requestAnimationFrame, integrates elapsed time between frames, and eases changes between Calm and Dynamic speeds. The field pauses when the document is hidden. The field is decorative and appears only at the larger hero breakpoint.

Dark mode uses a deep teal surface and luminous lines. Light mode has a pale green-white surface, darker teal particles and reduced shadows. The component reads `data-theme` during each render, so theme changes repaint without recreating application state. The theme-specific overlays are in `styles/main.css`.

When `prefers-reduced-motion` is active, the flow field renders a static frame and the reveal component immediately exposes its content. Do not replace the frame scheduler with `setInterval` for the canvas or animate invisible essential content.

## E2E

`tests/hero_flow.rs` checks canvas initialization, fresh seeds on reload, the information disclosure and reduced-motion presentation. `tests/journeys.rs` emulates reduced motion in Chromium. `tests/visual.rs` captures screenshots after the motion has settled. No screenshot comparison is considered verified unless an approved baseline has been supplied.
