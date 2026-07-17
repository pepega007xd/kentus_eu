# kentus.eu

### ⚠️ YES, THIS IS COMPLETELY VIBECODED, I AM NOT DOING WEBDEV BY HAND ⚠️

Static SvelteKit SPA reimplementation of [kentus.eu](https://kentus.eu) — a personal home dashboard showing indoor/outdoor sensor readings (temperature, humidity, pressure) with history charts, plus a Typst table generator utility. It is a pure client for the public REST API at `https://api.kentus.eu`; no backend ships with this repo.

See `specification.md` for the framework-agnostic functional spec and `plan.md` for the original implementation plan. The implementation diverges from the plan in a few areas (dark theme, i18n, expanded links); this README documents what is actually in the repo.

## Tech stack

- **SvelteKit 2** (SPA mode via `@sveltejs/adapter-static`, `fallback: 'index.html'`) — no server runtime, output is plain static files
- **Svelte 5** using runes (`$props`, `$state`, `$derived`, `$effect`)
- **TypeScript** (strict) checked with `svelte-check`
- **Vite 5** as the build tool
- **Chart.js 4** + `chartjs-adapter-date-fns` for the sensor history line charts (multi-axis, time x-axis, legend toggle)

## Project structure

```
src/
├── app.html                      # HTML shell, manifest link, lang="en"
├── app.css                       # Global dark theme (CSS variables), layout, components
├── app.d.ts                      # Svelte/Vite type references
├── routes/
│   ├── +layout.svelte            # Imports app.css, renders children
│   ├── +page.svelte              # Home dashboard (header, locale toggle, links, readings, charts)
│   └── typst_table/+page.svelte  # Typst table generator page
└── lib/
    ├── types.ts                  # SensorReading, HistoryResponse, SensorLocation, ChartVariable
    ├── api.ts                    # fetch wrappers around https://api.kentus.eu (hardcoded BASE_URL)
    ├── i18n.ts                   # en/cs locale store with localStorage persistence + auto-detect
    ├── colors.ts                 # Reads CSS variables for chart colors
    ├── components/
    │   ├── CurrentReadings.svelte  # Latest indoor/outdoor reading card (+ pressure for outdoor)
    │   ├── SensorChart.svelte      # Chart.js line chart with 1/3/7-day selector and legend toggle
    │   └── TypstTable.svelte       # Reactive tab-separated → Typst `#table(...)` converter
    └── utils/
        ├── format.ts             # formatTemperature / formatHumidity / formatPressure
        └── typst.ts              # convertToTypstTable (pure function, CellType = 'brackets' | 'dollars')

static/
├── manifest.json                 # PWA manifest
└── icons/                        # nextcloud.svg, pufferpanel.png, qbittorrent.svg, typst.svg

svelte.config.js                  # adapter-static, SPA fallback
vite.config.ts                    # SvelteKit plugin only
tsconfig.json                     # strict, extends .svelte-kit/tsconfig.json
```

## Routes

| Path           | Page                                                              |
|----------------|-------------------------------------------------------------------|
| `/`            | Home dashboard: links, current readings, indoor+outdoor charts   |
| `/typst_table` | Typst table generator (no API calls, pure client-side)            |

## API

All calls go to `https://api.kentus.eu` (CORS `*`, no auth):

- `GET /temperature` — latest indoor reading
- `GET /outdoor_temperature` — latest outdoor reading
- `GET /temperature/history?count=N` — indoor history (newest first; `api.ts` reverses it for charts)
- `GET /outdoor_temperature/history?count=N` — outdoor history

`count` defaults to 1440 (24h at 1-min intervals). The chart component requests `selectedDays * 1440`. Pressure comes from the API in Pascals and is divided by 100 for display (hPa). See `specification.md` §6 for the full API contract and DB schema.

## Charts

`SensorChart.svelte` renders three datasets (temperature, humidity, pressure) on a single Chart.js line chart with three y-axes. Temperature is visible by default; humidity and pressure are hidden, and their y-axes are hidden until at least one dataset using them is shown. The legend click handler (`onLegendClick`) toggles dataset visibility and axis display. The x-axis is a `time` scale; tooltip format is `dd.MM. HH:mm`. A 1/3/7-day button group re-fetches with the corresponding `count`.

## i18n

`src/lib/i18n.ts` exports a Svelte store `locale` (`'en' | 'cs'`), a `setLocale` setter that persists to `localStorage`, and a derived `t` translation function. The detected locale defaults from `navigator.language` (Czech → `cs`, otherwise `en`). An EN/CS toggle in the home page header switches at runtime; `SensorChart` also re-fetches/rebuilds on locale change so axis labels translate.

## Build & development

```bash
npm install
npm run dev          # Vite dev server
npm run build        # Static SPA output in build/
npm run preview      # Serve the build/ output locally
npm run check        # svelte-check type checking
```

`npm run build` produces a static `build/` directory (SPA fallback `index.html` + hashed `_app/` chunks + copied `static/` assets). Deploy by copying `build/` to any static web server and configuring a fallback to `index.html` for client-side routes.
