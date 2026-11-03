# GitHub Pages deployment

## Build output

From the repository root:

```powershell
cargo make release
```

`crates/web/Trunk.toml` sets `dist = "../../dist"`, so the build outputs to repository-root `dist/`. Trunk copies runtime JSON, project images, browser icons, the portrait, wallpaper and the public CV according to `crates/web/index.html`.

## GitHub Actions

`.github/workflows/deploy.yml` runs on pushes to `master` and through manual workflow dispatch. It checks out the repository, caches Cargo, installs Trunk tooling and the WASM target, runs `trunk build --release` from `crates/web`, uploads `./dist/` and deploys it to GitHub Pages. There is no Node.js/npm step and **no `CNAME` copy step** in this workflow. The current workflow does not publish the root `CNAME` file. If a custom domain is needed, check the Pages settings and ensure the required domain setup is managed deliberately.

## Release checklist

1. Run `cargo make verify`, `cargo make e2e` and `cargo make release`.
2. Review the public PDF for personal contact details and confidential employer or client information.
3. Check `dist/data/`, `dist/images/`, `dist/icons/` and the copied PDF.
4. Confirm both themes, project images and Canvas visuals in a browser, including a reduced-motion setting.
5. After deployment, check that the JSON responses have HTTP success status, WebP images decode, the CV opens and deep links such as `/#/projects/sudoku` work.
6. Check the GitHub Actions build/deploy job results and the actual published site.

## Search and social preview metadata

`index.html` supplies a title, description, canonical homepage URL, Open Graph and Twitter preview metadata, a small Person JSON-LD record and established favicon links. `robots.txt` and `sitemap.xml` are copied to `dist/` by Trunk. The canonical, sharing image and sitemap URLs currently target `https://jozefpodlecki.github.io/`; adjust them together if the published origin changes. Verify the metadata and its preview image after deployment.

HashRouter case studies (`/#/projects/:slug`) are not independent crawlable documents. The sitemap deliberately lists only the homepage; unique search-result metadata for each project would require static prerendering or a routing strategy that serves separate HTML documents. See [SEO.md](SEO.md).

## Routing and limitations

The Yew `HashRouter` uses `/#/route`, so static hosting does not need server-side route rewrites. Runtime assets are requested by relative paths. Do not test the built application using `file://`: serve the distribution over HTTP. The local production asset E2E test verifies `dist/`, not the GitHub Pages CDN; use the optional deployed-site test for a real deployment check.

Public JSON and bundled static assets should contain no credentials or information you do not intend to publish.
