# Release quality checklist

## Automated

- [ ] `cargo make verify` completes without errors or Clippy warnings.
- [ ] `cargo test -p portfolio-e2e --tests --no-run` compiles every E2E target.
- [ ] `cargo make e2e` passes the browser and fixture suites.
- [ ] `cargo make release` builds the release bundle.
- [ ] If using visual baselines, review differences before approving updates.

## Manual review

- [ ] Home, About, Projects, three case studies and unknown route display correctly.
- [ ] Light and dark themes have readable text and visually balanced Canvas rendering.
- [ ] The animated hero pauses or becomes static under reduced-motion settings.
- [ ] Images load normally; a missing project image has a deliberate fallback.
- [ ] Project priority is respected in Home and Projects; test pagination with more than nine entries and confirm filtering resets page one.
- [ ] The footer repository link, public canonical URL, Open Graph image and generated sitemap all resolve correctly.
- [ ] Keyboard traversal, skip link, filter controls, map dialog focus trap, Escape and restored focus work.
- [ ] Theme preference survives navigation and refresh.
- [ ] Core/profile failures show a retry UI instead of a permanent spinner.
- [ ] The fullscreen photo viewer displays the unfiltered Madeira image, switches between Fill screen and Entire photo without affecting the current route, and closes with Back, click-away and Escape while restoring focus.
- [ ] No unintentional horizontal overflow at narrow mobile widths or 200% zoom.
- [ ] The public CV, favicons and downloadable assets work in the built site.
- [ ] The public JSON and PDF contain no private contact details or credentials.
- [ ] Published asset URLs and case-study hash routes work on GitHub Pages.

The suite checks automated behaviour; passing tests does not guarantee full screen-reader compliance or correct production hosting. Review the live site after deployment.

## Education and travel data

Verify that the About page shows the University of Warmia and Mazury in Olsztyn and that the travel map marks the United Arab Emirates. The Poland tooltip should describe its significance as the country of origin without inventing a visit date.

## Connection notice

The application keeps rendered content visible when connectivity is lost. A non-modal, dismissible notice appears on browser `offline` events. On localhost, it also checks `index.html` about every seven seconds, because stopping Trunk does not necessarily change `navigator.onLine`. The notice offers a Try again action and disappears automatically when connectivity is restored unless dismissed. The health polling is deliberately restricted to localhost and 127.0.0.1, avoiding repeated production requests. Individual content-fetch errors still use their existing inline recovery UI.
