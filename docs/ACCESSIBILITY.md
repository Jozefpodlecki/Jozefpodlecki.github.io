# Accessibility

The portfolio uses semantic links, buttons, landmarks and keyboard focus indicators. `data-component` attributes support E2E inspection but are not substitutes for accessible names or native semantics.

## Labels and state

Yew supports standard `aria-label`, `aria-labelledby`, `aria-expanded`, `aria-controls`, `aria-pressed` and other ARIA attributes in `html!`. Prefer a visible label where possible; give icon-only controls explicit accessible names. The theme toggle, photo disclosure, filters and map expansion controls expose state where appropriate.

## Keyboard behaviour

The skip-navigation link moves focus to the main route-content element without disrupting hash-based routing. The fullscreen travel map focuses its close control, keeps Tab navigation inside the dialog, closes with Escape and restores focus to Expand. Visited countries respond to keyboard focus and activation; unvisited countries are not keyboard actions. The fullscreen photograph dialog focuses Back to portfolio when opened, cycles focus between its two controls, prevents background scrolling, closes via Back, outside click or Escape, and restores focus to the camera button.

## Motion and decorative content

`MotionReveal` immediately exposes content when reduced motion is requested. The Canvas hero renders one static frame instead of running its animation loop and is `aria-hidden` because it conveys no essential information. The native cursor and visible focus outlines remain. The cursor glow is restricted to dark-mode fine-pointer devices without reduced motion.

## Content, selection and context menu

Presentation headings, visual cards and experience entries may use `select-none` to prevent incidental highlights. Case-study text remains selectable. In debug builds the browser context menu is available; in release builds it is suppressed globally in BrowserShell as a presentation choice; it does not prevent downloading publicly served assets or reading page content.

## Manual checks

Automated tests cover major keyboard journeys and reduced-motion preference. Before release also test a screen reader, zoom to 200%, narrow mobile viewports, colour contrast and focus visibility against both themes. Test actual touch interactions and the readability of the travel country summaries. Passing ARIA assertions alone does not establish WCAG conformance.
