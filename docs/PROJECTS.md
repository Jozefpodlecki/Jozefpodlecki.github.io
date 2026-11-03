# Project case-study references

Project content comes from `crates/web/public/data/projects.json` and is fetched at runtime. The descriptions reflect the public source that was reviewed when these entries were created. Confirm features against the current repositories before making new claims.

## Sudoku

- Repository: https://github.com/Jozefpodlecki/Sudoku
- Published demo listed in portfolio: https://jozefpodlecki.github.io/Sudoku/
- Described capabilities: independent Rust puzzle engine, generation, uniqueness checks, Algorithm X solving, logical hints and difficulty ratings.

## win-drives

- Repository: https://github.com/Jozefpodlecki/win-drives
- Described capabilities: Windows disk/volume enumeration, NT API integration, geometry and partition information, aligned reads and optional no-std support.
- No live web demo is listed because this is a library.

## Zipeek

- Repository: https://github.com/Jozefpodlecki/zipeek
- Described public capabilities: Rust/Yew frontend, DuckDB-WASM dictionary and vocabulary practice modes.
- Ongoing work may not be reflected in the public repository; do not advertise unpushed features as released.

## Ordering and pagination

Assign a numeric `priority` to each object in `projects.json`. Lower numbers appear earlier across Home and Projects. The current three projects use `10`, `20` and `30`, respectively. Entries without a priority default to `100`; equal priority values preserve their JSON array order. The homepage always shows three featured projects. The Projects gallery shows nine matching cards per page with accessible page navigation whenever more than nine results remain. Technology filtering resets the gallery to page one. Add or reorder projects in JSON without changing Yew code.

## Artwork and fallback

The three project illustrations are locally bundled WebP assets under `public/images/`. They are conceptual artwork, not screenshots of the real applications. The `image` property in JSON determines which image is displayed. The shared `ProjectArtwork` component reserves the media area and shows an accessible fallback if an asset is absent or fails to decode. E2E tests exercise both a successful image response and an intentionally missing asset.

Replace artwork with accurate application screenshots when available. External repository and live-site URLs are optional and should not be invented.
