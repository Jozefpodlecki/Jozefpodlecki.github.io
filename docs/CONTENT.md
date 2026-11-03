# Runtime content and public assets

The JSON files in `crates/web/public/data/` are copied into `dist/data/` and fetched over HTTP. Their values are public. Never store tokens, private addresses, phone numbers or confidential employer data in the static distribution.

## JSON files

| File | Content |
| --- | --- |
| `profile.json` | `full_name`, `role`, `intro`, social URLs, optional About paragraphs, education entries and photo metadata |
| `experience.json` | Company, location, role, period, optional employment details and responsibility bullets |
| `projects.json` | Name, slug, numeric `priority`, summary, tags, narrative, repository URL, optional deployed URL and local artwork |
| `travel.json` | ISO alpha-3 country code, optional year and optional location/note; multiple entries per country are supported |

Education entries in `profile.json` use `qualification`, `institution`, `location`, and `period`, with optional `website` and `logo`. The `education` array is optional and renders below work experience on the About page. The university links to its official website, and the supplied logo is served as a local image under `public/images/`.

`profile.json` requires `full_name`, `role`, `intro` and the GitHub and LinkedIn social addresses. `experience.json` uses `company`, `location`, `role`, `period` plus optional `employment_type`, `work_mode` and `responsibilities` fields. A project uses `name`, `description`, `slug` and optional `priority`, `tags`, `body`, `github`, `link` and `image`. Empty link values hide external actions. Slugs are used for `/#/projects/:slug` case-study routes. Projects are sorted by ascending `priority` (lower numbers appear first; default `100`). Equal priorities preserve JSON order. The Home page shows the first three projects; the Projects page filters before paginating, nine projects per page. Pagination resets to the first page whenever a technology filter changes.

Travel entries require `code`, matching the bundled country geometry. `year` and `note` can be omitted. Set `kind` to `"transit"` for airport-only connections; it defaults to a destination visit. Madeira (Miradouro da Ponta do Rosto, 2026) is represented under `PRT`, and Germany has separate records for Hamburg and Frankfurt. Missing years should remain absent rather than guessed. Poland is noted as the country of origin and former home; Dubai Airport (2026) is represented under `ARE` as `"kind": "transit"`, as is Frankfurt Airport in Germany.

## Recovery and validation

A failure to load essential profile or experience content presents the core recovery UI. Project and travel loading remain independent, so their failures do not block unrelated pages. Content fetches and validation produce typed errors; visitors see concise recovery messages and an actionable retry.

## Public assets

- Project illustration files: `crates/web/public/images/`, referenced by project `image` paths.
- Portrait and wallpaper: `crates/web/public/avatar.jpg` and `wallpaper.jpg`. The map-pin "About this place" button appears from the 640px breakpoint and opens the same wallpaper image without the page-wide colour overlays; the photo viewer reads location, photographer, year and description from `profile.json`.
- Browser identity: `crates/web/public/icons/`, copied by Trunk.
- Public CV: `crates/web/public/Jozef_Podlecki_Resume_2026_11_03_Public.pdf`.

When renaming the CV, update both the Trunk `copy-file` declaration in `crates/web/index.html` and the link in the relevant Yew component. The public PDF should be reviewed before release for accidental personal or confidential information.

The current project entries describe Sudoku, win-drives and Zipeek based on public repository evidence. Zipeek may contain unpushed work, so site claims should follow verifiable public features. See [PROJECTS.md](PROJECTS.md).
