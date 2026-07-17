# Implementation Plan — kentus_eu

Reimplementation of kentus.eu as a static SvelteKit SPA with expanded chart functionality.

---

## 1. Tech Stack

| Concern             | Choice                                      | Rationale                                                     |
|---------------------|---------------------------------------------|---------------------------------------------------------------|
| Framework           | SvelteKit 2.x                               | Compiles away to vanilla JS; smallest output; built-in reactivity |
| Language            | TypeScript                                  | Type safety for API models and component props                |
| Build tool          | Vite (via SvelteKit)                         | Fast dev server; optimized static production build             |
| Static adapter      | `@sveltejs/adapter-static`                  | Outputs a folder of static HTML/CSS/JS — no server runtime     |
| Charting            | Chart.js 4.x + chartjs-adapter-date-fns    | Built-in legend toggle, multi-axis, hover tooltips, time scale |
| Deployment          | Static files (copy to any web server)       | No Node.js runtime needed                                     |

### Chart.js features used

- **Multi-series:** temperature, humidity, and pressure as separate datasets in a single chart
- **Toggle visibility:** Chart.js built-in legend (click to show/hide a series)
- **Multi-day range:** a dropdown/buttons selector that changes the `count` query parameter sent to the API (1 day = 1440, 3 days = 4320, 7 days = 10080)
- **Hover tooltips:** Chart.js built-in tooltip plugin showing the exact value(s) at the cursor position
- **Time x-axis:** `chartjs-adapter-date-fns` for a proper time scale on x-axis
- **Multiple y-axes:** one axis per unit (°C, %, hPa) so variables with different ranges display correctly

### Key dependencies (package.json)

```
devDependencies:
  @sveltejs/adapter-static
  @sveltejs/kit
  svelte
  svelte-check
  typescript
  vite

dependencies:
  chart.js
  chartjs-adapter-date-fns
```

---

## 2. Project Configuration

### SvelteKit config (`svelte.config.js`)
- Use `adapter-static`
- Set SPA mode: `fallback: "index.html"` (so client-side routing works without a server)
- Prerender is off (pure CSR, matching the original)

### TypeScript config
- Strict mode enabled
- `svelte-check` for type checking Svelte components

### Vite config
- Default SvelteKit Vite config; no custom plugins needed
- Chart.js is tree-shakeable but we import the full build for simplicity (can optimize later by registering only needed components)

---

## 3. Architecture

### 3.1 Directory Structure

```
kentus_eu/
├── src/
│   ├── app.html                        # HTML shell (meta, manifest link, title)
│   ├── app.css                         # Global styles (textarea sizing, etc.)
│   │
│   ├── routes/
│   │   ├── +layout.svelte              # Root layout (renders children)
│   │   ├── +page.svelte                # Home dashboard page
│   │   └── typst_table/
│   │       └── +page.svelte            # Typst table generator page
│   │
│   └── lib/
│       ├── types.ts                    # Shared TypeScript types
│       ├── api.ts                      # API client (fetch wrappers)
│       │
│       ├── components/
│       │   ├── CurrentReadings.svelte  # Current temp/humidity/pressure display
│       │   ├── SensorChart.svelte       # Reusable Chart.js chart component
│       │   └── TypstTable.svelte        # Typst table generator (input/output/cell type)
│       │
│       └── utils/
│           ├── format.ts               # Number formatting helpers
│           └── typst.ts                # Typst table conversion logic
│
├── static/
│   └── manifest.json                   # PWA manifest
│
├── svelte.config.js
├── vite.config.ts
├── tsconfig.json
├── package.json
├── specification.md                    # Functional specification (existing)
└── plan.md                             # This file
```

### 3.2 Module Responsibilities

#### `src/lib/types.ts`
TypeScript interfaces for all API data models:

```typescript
// Single sensor reading (current or history entry)
interface SensorReading {
  temperature: number;
  pressure: number;      // Pascals from API
  humidity: number;      // Percentage
  record_timestamp: string;  // ISO 8601, no timezone
}

// History endpoint response
interface HistoryResponse {
  values: SensorReading[];
}

// Which sensor location to query
type SensorLocation = "indoor" | "outdoor";

// Which variables to plot
type ChartVariable = "temperature" | "humidity" | "pressure";
```

#### `src/lib/api.ts`
Thin fetch wrappers around the API. All functions return promises. Base URL hardcoded as `https://api.kentus.eu`.

```
getCurrentReading(location: SensorLocation): Promise<SensorReading>
getHistory(location: SensorLocation, count: number): Promise<SensorReading[]>
```

- `getCurrentReading` calls `/temperature` or `/outdoor_temperature`
- `getHistory` calls `/temperature/history?count=N` or `/outdoor_temperature/history?count=N`
- `getHistory` reverses the result array (API returns newest-first; charts need oldest-first)
- Each function throws on network/parse error (the caller decides how to handle it)

#### `src/lib/utils/format.ts`
Pure formatting functions:

```
formatTemperature(value: number): string   // "22.50 ˚C" (2 decimals)
formatHumidity(value: number): string      // "45.2%" (1 decimal)
formatPressure(valuePa: number): string    // "1013.3 hPa" (Pa / 100, 1 decimal)
```

#### `src/lib/utils/typst.ts`
The Typst table conversion logic (pure function, no Svelte/DOM dependencies):

```
type CellType = "brackets" | "dollars";

convertToTypstTable(input: string, cellType: CellType): string
```

Implements the algorithm from the specification (Section 5.2):
1. Replace all commas with periods
2. Count columns from first line (split by tab)
3. Wrap each cell in `[]` or `$$` based on cellType
4. Assemble into `#table(columns: N, ...cells)` format

#### `src/lib/components/CurrentReadings.svelte`
Displays the current indoor or outdoor readings. Props:

```
location: SensorLocation
```

- On mount, calls `api.getCurrentReading(location)`
- Shows "Loading..." while fetching
- Shows formatted temperature + humidity (and pressure for outdoor)
- Shows "Error" if the fetch fails (improvement over original which was stuck on Loading)

#### `src/lib/components/SensorChart.svelte`
The reusable chart component. Props:

```
location: SensorLocation           // indoor | outdoor → determines API endpoint
title: string                      // Chart heading text
```

Internal state:
- `selectedDays: number` — how many days of history to show (default: 1). Options: 1, 3, 7. Changing this re-fetches data.
- Chart.js instance managed via a canvas ref and `onMount`/`onDestroy` lifecycle.

Data flow:
1. On mount (and when `selectedDays` changes), compute `count = selectedDays * 1440`
2. Call `api.getHistory(location, count)`
3. Transform `SensorReading[]` into Chart.js datasets:
   - Extract timestamps → x-axis (converted to JS `Date` objects)
   - Three datasets: temperature (°C), humidity (%), pressure (hPa)
   - Each dataset assigned to its own y-axis (`yTemp`, `yHumidity`, `yPressure`)
4. Render Chart.js line chart with:
   - Type: `line`
   - X-axis: `time` scale (via chartjs-adapter-date-fns)
   - Three y-axes: one per unit, each with appropriate label
   - Legend: enabled (click to toggle series visibility — built-in)
   - Tooltips: enabled (shows all variable values at hovered time — built-in `mode: "index"`)
   - Loading state: show "Loading..." text before data arrives
   - Error state: show error message if fetch fails

UI layout:
```
[h3 title]
[1 day] [3 days] [7 days]     ← day selector buttons
<canvas>                       ← Chart.js chart
```

#### `src/lib/components/TypstTable.svelte`
The Typst table generator. No props (self-contained).

- Reactive `input` string bound to the input textarea
- Reactive `cellType` (`"brackets" | "dollars"`) bound to radio buttons
- Computed `output` = `convertToTypstTable(input, cellType)` (updates live via Svelte reactivity)
- Two textareas (input + readonly output) and radio buttons, per specification Section 5

#### `src/routes/+layout.svelte`
Minimal root layout — just renders `<slot />` for child routes. No nav bar (the original has none).

#### `src/routes/+page.svelte` (Home)
Assembles the home page:
1. Header (h1, italic subtitle)
2. Links sections (general + LAN-only)
3. `<CurrentReadings location="indoor" />`
4. `<CurrentReadings location="outdoor" />`
5. `<SensorChart location="indoor" title="Indoor temperature" />`
6. `<SensorChart location="outdoor" title="Outdoor temperature" />`

#### `src/routes/typst_table/+page.svelte`
Renders `<TypstTable />` wrapped with the h2 heading.

#### `src/app.html`
HTML shell matching the original:
- `<html lang="en">`
- Title: `kentus.eu`
- Viewport meta tag
- `<link rel="manifest" href="manifest.json">`
- Global `<style>` for textarea sizing (or moved to `app.css`)

#### `src/app.css`
Global styles carried over from the original:
```css
textarea {
  display: block;
  width: 75vh;
  height: 40vh;
}
```

#### `static/manifest.json`
PWA manifest (copied from original, unchanged).

---

## 4. Data Flow Diagram

```
User opens /
  │
  ├── +page.svelte mounts
  │     │
  │     ├── CurrentReadings location="indoor"
  │     │     └── api.getCurrentReading("indoor")
  │     │           └── GET https://api.kentus.eu/temperature
  │     │
  │     ├── CurrentReadings location="outdoor"
  │     │     └── api.getCurrentReading("outdoor")
  │     │           └── GET https://api.kentus.eu/outdoor_temperature
  │     │
  │     ├── SensorChart location="indoor" title="Indoor temperature"
  │     │     └── api.getHistory("indoor", 1440)
  │     │           └── GET https://api.kentus.eu/temperature/history?count=1440
  │     │
  │     └── SensorChart location="outdoor" title="Outdoor temperature"
  │           └── api.getHistory("outdoor", 1440)
  │                 └── GET https://api.kentus.eu/outdoor_temperature/history?count=1440

User clicks [3 days] on a chart
  │
  └── SensorChart re-fetches with count = 4320

User opens /typst_table
  │
  └── TypstTable mounts
        └── No API calls; pure client-side reactive computation
```

---

## 5. Implementation Order

1. **Scaffold project:** `npm create svelte@latest`, install deps, configure `adapter-static` + SPA fallback
2. **Static assets:** Copy `manifest.json` to `static/`, set up `app.html` and `app.css`
3. **Types + API client:** Create `types.ts` and `api.ts`
4. **Format utils:** Create `format.ts`
5. **Typst module:** Create `typst.ts` (pure function, testable in isolation)
6. **Typst table page:** Build `TypstTable.svelte` + route — simplest page, validates the routing setup
7. **CurrentReadings component:** Build and test with real API
8. **SensorChart component:** Build Chart.js integration with single series (temperature only first)
9. **Expand chart:** Add humidity + pressure datasets, multiple y-axes, legend toggle
10. **Day selector:** Add the 1/3/7 day buttons, wire to re-fetch
11. **Tooltips:** Configure Chart.js tooltip (`mode: "index"`, `intersect: false`) to show all values at hover
12. **Home page:** Assemble all components on `+page.svelte`
13. **Type check + build:** Run `svelte-check` and `npm run build`, verify static output

---

## 6. Chart.js Configuration Reference

The `SensorChart` component will configure Chart.js roughly as follows:

```
type: "line"
data:
  labels: [Date, Date, ...]   (parsed from record_timestamp)
  datasets:
    - label: "Temperature (°C)",  data: [...],  yAxisID: "yTemp"
    - label: "Humidity (%)",      data: [...],  yAxisID: "yHumidity"
    - label: "Pressure (hPa)",    data: [...],  yAxisID: "yPressure"

options:
  scales:
    x:
      type: "time"
      time: { unit: "hour" }   (adapts to range)
    yTemp:      { position: "left",   title: "Temperature (°C)" }
    yHumidity:  { position: "right",  title: "Humidity (%)" }
    yPressure:  { position: "right",  title: "Pressure (hPa)" }

  plugins:
    legend: { display: true }                    ← click to toggle series
    tooltip:
      mode: "index"                              ← shows all series at hover x
      intersect: false                           ← tooltip follows cursor, not nearest point

  interaction:
    mode: "index"
    intersect: false
```

---

## 7. Build & Deploy

### Build command
```
npm run build
```

### Output
Static files in `build/` (default adapter-static output directory):
```
build/
├── index.html          ← SPA fallback
├── _app/               ← JS/CSS chunks (hashed)
├── manifest.json       ← PWA manifest (copied from static/)
└── ...
```

### Deploy
Copy the entire `build/` folder to any static web server. No Node.js runtime required. Configure the web server to serve `index.html` as a fallback for all routes (for client-side routing).
