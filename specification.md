# kentus.eu Website Specification

Comprehensive specification of all functionality in the kentus.eu website and its companion API (api.kentus.eu). This document is framework-agnostic and intended to allow a full reimplementation of the website in any JS framework.

---

## 1. Application Overview

**kentus.eu** is a client-side rendered single-page application (SPA) that serves as a personal home dashboard. It displays indoor and outdoor temperature/humidity/pressure readings (sourced from a REST API), renders temperature history line charts, provides a Typst table generator utility, and hosts a set of personal links.

The website is a PWA (Progressive Web App) with a web manifest.

### Original Tech Stack (for reference)
- **Frontend:** Rust + Leptos (CSR mode) compiled to WebAssembly, bundled with Trunk
- **Graphing:** plotters + plotters-canvas
- **HTTP client:** reqwest (wasm)
- **Backend API:** Rust + Rocket, SQLite via sqlx

### Key URLs
- Website: `https://kentus.eu`
- API: `https://api.kentus.eu`

---

## 2. PWA Manifest

The application includes a web manifest (`manifest.json`):

| Field             | Value        |
|-------------------|--------------|
| name              | kentus.eu    |
| short_name        | kentus.eu    |
| theme_color       | #000000      |
| background_color  | #000000      |
| display           | browser      |
| scope              | /            |
| start_url         | /            |

---

## 3. Routing

The application uses client-side routing with the following routes:

| Path           | Page / Action         |
|----------------|-----------------------|
| `/`            | Home dashboard        |
| `/typst_table` | Typst table generator |

---

## 4. Home Page (`/`)

The home page is the main dashboard. It contains several sections rendered top to bottom.

### 4.1 Header
- H1 heading: `kentus.eu`
- Italic text: `Now with 400% more wasm!`

### 4.2 Links Section
H3 heading: `Some links`

| Label                   | URL                                |
|-------------------------|------------------------------------|
| File Browser            | https://filebrowser.kentus.eu      |
| Typst table generator   | /typst_table (internal route)      |

### 4.3 LAN-Only Links Section
H3 heading: `LAN only`

| Label        | URL                              |
|--------------|----------------------------------|
| qBittorrent  | https://qbt.kentus.eu/           |
| PufferPanel  | https://pufferpanel.kentus.eu/    |
| Scanner      | https://scanner.kentus.eu/        |

### 4.4 Indoor Temperature (Current)
- H3 heading: `Temperature at home`
- Fetches latest reading from `GET https://api.kentus.eu/temperature`
- While loading, displays: `Loading...`
- Once loaded, displays: `{temperature} ˚C (relative humidity {humidity}%)`
  - Temperature formatted to 2 decimal places
  - Humidity formatted to 1 decimal place

### 4.5 Outdoor Temperature (Current)
- H3 heading: `Outdoor temperature`
- Fetches latest reading from `GET https://api.kentus.eu/outdoor_temperature`
- While loading, displays: `Loading...`
- Once loaded, displays: `{temperature} ˚C (relative humidity {humidity}%)`
  - Temperature formatted to 2 decimal places
  - Humidity formatted to 1 decimal place

- H3 heading: `Pressure`
- Uses the same fetched outdoor data
- While loading, displays: `Loading...`
- Once loaded, displays: `{pressure} hPa`
  - Pressure is the raw API value divided by 100, formatted to 1 decimal place
  - (The sensor reports pressure in Pascals; the display converts to hPa)

### 4.6 Indoor Temperature Graph (24h)
- H3 heading: `Indoor temperature (24h)`
- Fetches data from `GET https://api.kentus.eu/temperature/history`
- While loading, displays: `Loading...`
- Once loaded, renders a line chart of temperature over time, showing the last day by default

### 4.7 Outdoor Temperature Graph (24h)
- H3 heading: `Outdoor temperature (24h)`
- Fetches data from `GET https://api.kentus.eu/outdoor_temperature/history`
- While loading, displays: `Loading...`
- Once loaded, renders a line chart of temperature over time, showing the last day by default

---

## 5. Typst Table Generator (`/typst_table`)

A utility that converts tab-separated spreadsheet data (as produced by Ctrl+C from spreadsheet applications) into [Typst](https://typst.app/) table syntax.

### 5.1 UI Elements
1. **H2 heading:** `Insert a Ctrl-C selection from your favorite spreadsheet application, and this thing will convert it to a Typst table.`

2. **Cell type selector** (radio buttons):
   - Label paragraph: `How to insert data into table cells:`
   - Radio option 1 (default, pre-checked):
     - Value: `brackets`
     - Label: `square brackets (regular text)`
     - Wraps each cell as `[cell]`
   - Radio option 2:
     - Value: `dollars`
     - Label: `dollars (equation)`
     - Wraps each cell as `$cell$`

3. **Input textarea:**
   - Placeholder: `Input: `
   - User pastes tab-separated data here
   - On every input event, the output is recalculated

4. **Output textarea:**
   - Placeholder: `Output: `
   - Readonly
   - Displays the generated Typst table code

### 5.2 Conversion Algorithm

Given raw input text and selected cell type:

1. **Replace all commas** (`,`) in the input with periods (`.`).
2. **Count columns** from the first line by splitting on tab characters (`\t`).
3. **For each line** in the input:
   - Split the line by tab characters (`\t`).
   - For each cell element, wrap it according to the selected cell type:
     - Square brackets mode: `[` + element + `]`
     - Dollars mode: `$` + element + `$`
   - Join all wrapped cells with `, ` (comma + space).
   - Prepend two spaces (`  `) of indentation.
   - Append a newline (`\n`).
4. **Assemble the final output** in this format:

```
#table(
  columns: {columns},
{cells})
```

Where:
- `{columns}` is the column count from step 2.
- `{cells}` is the concatenation of all converted lines from step 3.

### 5.3 Conversion Example

Input (tab-separated):
```
1   2   3
4   5   6
```

Output (square brackets mode):
```
#table(
  columns: 3,
  [1], [2], [3], 
  [4], [5], [6], 
)
```

Output (dollars mode):
```
#table(
  columns: 3,
  $1$, $2$, $3$, 
  $4$, $5$, $6$, 
)
```

---

## 6. API Specification (api.kentus.eu)

The backend is a REST API backed by a SQLite database. All responses include the CORS header `Access-Control-Allow-Origin: *`. The API listens on port 8002.

### 6.1 Data Models

#### Temperature Reading (API response / database row)
All sensor endpoints share this data model:

| Field              | Type      | Description                                      |
|--------------------|-----------|--------------------------------------------------|
| `temperature`      | number (f64) | Temperature in degrees Celsius                 |
| `pressure`         | number (f64) | Atmospheric pressure in Pascals (hPa × 100)    |
| `humidity`         | number (f64) | Relative humidity as a percentage              |
| `record_timestamp` | string    | ISO 8601 datetime (no timezone), e.g. `2024-04-06T15:42:26` |

#### Sensor Input (POST body)
The POST sensor endpoints accept a JSON body with `temperature`, `pressure`, and `humidity`. The `record_timestamp` is **not** accepted from the client — it is automatically set to the current server time by the database.

```json
{
  "temperature": 22.5,
  "pressure": 101325.0,
  "humidity": 45.2
}
```

#### History Response
```json
{
  "values": [
    { "temperature": 22.5, "pressure": 101325.0, "humidity": 45.2, "record_timestamp": "2024-04-06T15:42:26" },
    ...
  ]
}
```

### 6.2 Endpoints

#### POST `/indoor_sensor`
Records a new indoor sensor measurement.

- **Request body:** JSON (`temperature`, `pressure`, `humidity` — `record_timestamp` is ignored)
- **Action:** Inserts a new row into the `temperature` table with `record_timestamp = CURRENT_TIMESTAMP`
- **Response:** Empty body (200 on success; if the DB is unavailable or the query fails, returns no response / 404)

#### POST `/outdoor_sensor`
Records a new outdoor sensor measurement.

- **Request body:** JSON (`temperature`, `pressure`, `humidity` — `record_timestamp` is ignored)
- **Action:** Inserts a new row into the `outdoor_temperature` table with `record_timestamp = CURRENT_TIMESTAMP`
- **Response:** Empty body (same behavior as indoor)

#### GET `/temperature`
Returns the latest indoor temperature reading.

- **Response:** JSON `Temperature Reading` object (the single most recent row by `record_timestamp DESC`)

#### GET `/temperature/history?count={N}`
Returns recent indoor temperature history.

- **Query parameter:** `count` (integer, optional, default `1440` = 24 hours × 60 minutes)
- **Response:** JSON `History Response` object containing up to `N` most recent rows, ordered by `record_timestamp DESC` (newest first)

#### GET `/outdoor_temperature`
Returns the latest outdoor temperature reading.

- **Response:** JSON `Temperature Reading` object (the single most recent row by `record_timestamp DESC`)

#### GET `/outdoor_temperature/history?count={N}`
Returns recent outdoor temperature history.

- **Query parameter:** `count` (integer, optional, default `1440` = 24 hours × 60 minutes)
- **Response:** JSON `History Response` object containing up to `N` most recent rows, ordered by `record_timestamp DESC` (newest first)

### 6.3 Database Schema

#### `temperature` table (indoor)
```sql
CREATE TABLE IF NOT EXISTS temperature (
    record_timestamp DATETIME PRIMARY KEY NOT NULL DEFAULT CURRENT_TIMESTAMP,
    temperature REAL NOT NULL,
    humidity REAL NOT NULL,
    pressure REAL NOT NULL DEFAULT 0.0
);
```

#### `outdoor_temperature` table
```sql
CREATE TABLE IF NOT EXISTS outdoor_temperature (
    record_timestamp DATETIME PRIMARY KEY NOT NULL DEFAULT CURRENT_TIMESTAMP,
    temperature REAL NOT NULL,
    pressure REAL NOT NULL,
    humidity REAL NOT NULL
);
```

**Key schema notes:**
- `record_timestamp` is the primary key and is automatically set to the server time on insert.
- The `temperature` (indoor) table originally had only `temperature` and `humidity`; `pressure` was added later with a default of `0.0`.
- The `outdoor_temperature` table was created with all three sensor columns from the start.

---

## 7. Global Styling

The application has minimal global styling defined in `index.html`:

```css
textarea {
  display: block;
  width: 75vh;
  height: 40vh;
}
```

- All `<textarea>` elements are block-level, with width and height set to `75vh` and `40vh` respectively (viewport-height-based sizing).
- The page has no other global CSS; components use default browser styling.

### Page Metadata
- `<html lang="en">`
- Title: `kentus.eu`
- Viewport: `width=device-width, initial-scale=1.0`
- Manifest linked via `<link rel="manifest" href="manifest.json">`

---

## 8. Component Behavior Summary

### 8.1 Data Fetching Pattern
All API calls use simple HTTP GET requests with no authentication, headers, or query parameters (except the optional `count` on history endpoints). Responses are parsed as JSON.

Each data-driven component follows this pattern:
1. On mount, initiate a fetch to the API endpoint.
2. Display `Loading...` while the request is in flight.
3. Once data arrives, parse the JSON and render the content.
4. If the request fails or returns no data, the component remains in the loading state (no error UI).

### 8.2 Reactivity
- The Typst table generator is fully reactive: the output textarea updates in real-time as the user types in the input textarea or changes the cell type radio button.
- The temperature/pressure display and graphs are fetched once on component mount (no polling or auto-refresh).

---

## 9. Complete Route → Endpoint Mapping

| Route      | Component             | API Endpoint(s) Called                                                        |
|------------|-----------------------|-------------------------------------------------------------------------------|
| `/`        | Home dashboard        | `GET /temperature`, `GET /outdoor_temperature`, `GET /temperature/history`, `GET /outdoor_temperature/history` |
| `/typst_table` | Typst table generator | None (purely client-side computation)                                    |

---

## 10. Reimplementation Notes for JS Frameworks

### 10.1 What You Need
- A JS SPA framework with client-side routing (React Router, Vue Router, SvelteKit, etc.)
- A charting library capable of line charts on canvas (Chart.js, uPlot, D3, or a custom canvas implementation)
- No backend is needed for the website itself — it is purely a client for api.kentus.eu.

### 10.2 Key Implementation Details
1. **Routing:** 2 routes total (`/` and `/typst_table`). Use route-level code splitting if desired.
2. **API base URL:** `https://api.kentus.eu` (hardcoded, no environment variable in original).
3. **History default count:** The history endpoints default to 1440 records (24 hours at 1-minute intervals) if no `count` query parameter is provided. The website does not pass a `count` parameter, so it always gets the default 24-hour window.
4. **Graph data ordering:** API returns newest-first; reverse to oldest-first before charting.
5. **Pressure conversion:** API returns pressure in Pascals; display divides by 100 to get hPa.
6. **Typst table:** Comma-to-period replacement happens before any other processing.
7. **No authentication:** All API endpoints are public; CORS allows all origins (`*`).

### 10.3 Testing Checklist
- [ ] Home page shows loading text, then current indoor/outdoor temperature, humidity, and pressure.
- [ ] Pressure display divides raw value by 100 and formats to 1 decimal.
- [ ] Temperature display formats to 2 decimals; humidity to 1 decimal.
- [ ] Both 24h graphs render as line charts showing the last day by default.
- [ ] Typst table generator converts tab-separated input in real-time.
- [ ] Typst table generator replaces commas with periods.
- [ ] Typst table cell type radio toggle switches between `[]` and `$$` wrapping.
- [ ] All external links on the home page point to the correct URLs.
- [ ] PWA manifest is served and linked correctly.
