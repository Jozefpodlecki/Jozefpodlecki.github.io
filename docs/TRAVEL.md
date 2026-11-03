# Travel map

The About page includes an interactive local SVG world map. Geometry is bundled in `crates/web/src/features/world_countries.json`, while visited places are fetched from `crates/web/public/data/travel.json` at runtime.

## Current places

The map marks Poland (`POL`), the United Kingdom (`GBR`), Cyprus (`CYP`), Germany (`DEU`), Croatia (`HRV`), Turkey (`TUR`), the Philippines (`PHL`), Portugal (`PRT`), Egypt (`EGY`), Venezuela (`VEN`), Australia (`AUS`), and the United Arab Emirates (`ARE`, airport transit only).

Dated travel records: Hamburg (2014), Cairo (2004), Venezuela (2005), New South Wales (2006), Antalya (2025), Croatia (2024), Paphos and Nicosia (2025), Cebu (2026), Madeira (2026), and Dubai Airport transit (2026). Frankfurt Airport was also a transit stop with no assigned year. Poland and the UK have no assigned visit dates. Madeira was visited in 2026. Poland is labelled as the country of origin, where Józef grew up and lived.

Germany has one visit (Hamburg) and one airport transit (Frankfurt). Dubai is an airport transit rather than a destination visit. The map distinguishes transit-only countries with a muted colour, and the tooltip marks airport connections with a plane icon. Madeira is shown using the Portugal outline since the map uses country-level geometry.

## Interaction

- Unvisited countries are map context only, with no pointer or keyboard selection.
- Visited countries display a tooltip on hover or focus; click, tap, Enter, or Space also reveals the tooltip.
- No separate country detail panel is rendered below the map.
- The Expand map button opens a viewport-sized map with narrow padding, no heading, a floating close button, and backdrop dismissal. Its tooltip uses the same complete visit records as the embedded map, but appears at the upper left with more room to avoid the close control.
- Screen reader users have labelled country buttons and an accessible expanded dialog.
- There is no country finder, zoom control, instructional placeholder label, or invented "not visited" detail.

## Content contract

```json
[
  {
    "code": "PRT",
    "year": 2026,
    "note": "Miradouro da Ponta do Rosto, Caniçal, Machico, Madeira, Portugal"
  }
]
```

`code` is an ISO alpha-3 code matching a geometry feature; `note` is visitor-facing text and `year` is optional. Set `kind` to `"transit"` for airport-only connections; omitted `kind` means a destination visit. More than one record may share the same country code. Do not add guessed visit dates.

Failures to load travel records show a compact retry control without blocking the About page. The geometry is built into the WASM binary, so only the visit records require an HTTP request.
