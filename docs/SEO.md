# Search and social metadata

The built homepage includes a descriptive title and summary, author and robots metadata, a canonical URL, Open Graph tags, a large-image Twitter card, a public homepage image and a minimal Person JSON-LD record. The existing favicon and Apple touch icon remain in place.

## Published origin

The HTML, `robots.txt` and `sitemap.xml` currently use `https://jozefpodlecki.github.io/`, the GitHub Pages origin. If you publish the site on a custom domain, update all absolute canonical, `og:url`, image, JSON-LD, robots sitemap and XML sitemap URLs together. Do not point canonical to a domain that does not serve the portfolio.

Trunk copies `public/robots.txt` and `public/sitemap.xml` to the distribution root. The XML sitemap intentionally includes the root URL only.

## Hash-route limitation

This is a Yew CSR application using `HashRouter`, with URLs such as `/#/projects/sudoku`. A fragment is not a separate HTTP document. Search engines and social preview bots are not guaranteed to discover or render each internal case study as an independently indexed page. Static HTML meta tags primarily describe the homepage. If independent project indexing becomes important, adopt prerendered case-study HTML or an appropriate server/static route strategy; adding hash routes to a sitemap alone is not an SEO solution.

## Final checks

- Verify that the public canonical URL is the one you actually intend to publish.
- Confirm `dist/robots.txt` and `dist/sitemap.xml` are included after the production build.
- Open the public preview image URL and check that the image is suitable for sharing.
- Review social previews after publishing and allow for crawler cache delays.
- Keep public project descriptions up to date, particularly work that is not yet pushed to GitHub.

References: [Google SEO starter guide](https://developers.google.com/search/docs/fundamentals/seo-starter-guide) and [JavaScript SEO basics](https://developers.google.com/search/docs/crawling-indexing/javascript/javascript-seo-basics).
