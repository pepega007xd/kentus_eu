use chrono::prelude::*;
use chrono::Duration;
use leptos::{html::Canvas, *};
use leptos_router::*;

mod typst_table;
use plotters::prelude::*;
use typst_table::TypstTable;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
struct Temperature {
    temperature: f32,
    humidity: f32,
    timestamp: DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct TemperatureHistory {
    values: Vec<Temperature>,
}

#[component]
fn Temperature() -> impl IntoView {
    let temperature = create_resource(
        || (),
        |_| async move {
            let json = reqwest::get("https://api.kentus.eu/temperature")
                .await
                .ok()?
                .text()
                .await
                .ok()?;
            let temperature: Temperature = serde_json::from_str(&json).unwrap();
            Some(temperature)
        },
    );

    view! {
        <h3>"Temperature at home"</h3>
        {move || match temperature.get().flatten() {
            None => "Loading...".into_view(),
            Some(t) => format!(
                    "{:.2} ˚C (relative humidity {:.1}%)",
                    t.temperature,
                    t.humidity)
                .into_view()
        }}
    }
}

fn create_graph(data: TemperatureHistory) -> Option<HtmlElement<Canvas>> {
    // calculated limits of the graph
    let data = data.values.iter().rev();
    let temp_min = data.clone().map(|t| t.temperature as i32 - 1).min()? as f32;
    let temp_max = data.clone().map(|t| t.temperature as i32 + 1).max()? as f32;
    let data_duration_minutes =
        (data.clone().last()?.timestamp - data.clone().next()?.timestamp).num_minutes();
    let first_sample_timestamp = data.clone().next()?.timestamp;

    // create the canvas
    let canvas = html::canvas();
    let orig_size = (canvas.width(), canvas.height());
    canvas.set_width(canvas.width() * 2);
    canvas.set_height(canvas.height() * 2);
    let canvas = canvas.attr("style", format!("width: {}px;", orig_size.0));
    let canvas2 = (*canvas).clone();
    let backend = plotters_canvas::CanvasBackend::with_canvas_object(canvas2)?;
    let root = backend.into_drawing_area();
    root.fill(&plotters::style::WHITE).ok()?;
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..data_duration_minutes, temp_min..temp_max)
        .ok()?;

    chart
        .configure_mesh()
        .x_labels(10)
        .x_label_formatter(&|v| {
            let utc_time = first_sample_timestamp + Duration::minutes(*v);
            let local_time: DateTime<Local> = utc_time.into();
            format!("{:00}:00", local_time.hour())
        })
        .y_label_formatter(&|v| format!("{v} ℃",))
        .y_labels(10)
        .max_light_lines(0)
        .draw()
        .ok()?;

    // draw the data
    chart
        .draw_series(LineSeries::new(
            data.clone().map(|t| {
                (
                    (t.timestamp - first_sample_timestamp).num_minutes(),
                    t.temperature as f32,
                )
            }),
            &RED,
        ))
        .ok()?;

    root.present().ok()?;

    Some(canvas)
}

#[component]
fn TemperatureGraph() -> impl IntoView {
    let data = create_resource(
        || (),
        |_| async move {
            let json = reqwest::get("https://api.kentus.eu/temperature/history")
                .await
                .ok()?
                .text()
                .await
                .ok()?;
            let temperature: TemperatureHistory = serde_json::from_str(&json).unwrap();
            Some(temperature)
        },
    );

    view! {
        <h3>"Temperature history (24 h)"</h3>
        {move || match data.get().flatten() {
            None => "Loading...".into_view(),
            Some(data) => create_graph(data).into_view()
        }}
    }
}

#[component]
fn home() -> impl IntoView {
    view! {
        <h1>"kentus.eu"</h1>
        <i>"Now with 400% more wasm!"</i>

        <h3>"Some links"</h3>
        <a href="https://filebrowser.kentus.eu">"File Browser"</a><br/>
        // _blank is so that browser makes a separate request -> nginx serves different file
        <a href="frama-c-api/" target="_blank">"Frama-C documentation"</a><br/>
        <a href="frama-c-server-api/" target="_blank">"Frama-C Server documentation"</a><br/>
        <a href="astral-api/" target="_blank">"Astral solver documentation"</a><br/>
        <a href="typst_table">"Typst table generator"</a>
        <br/>

        <h3>LAN only</h3>
        <a href="https://qbt.kentus.eu/">"qBittorrent"</a><br/>
        <a href="https://grafana.kentus.eu/">"Grafana"</a><br/>

        <Temperature/>
        <br/> <br/>
        <TemperatureGraph/>
    }
}

#[component]
fn rickroll() -> impl IntoView {
    leptos::web_sys::window()
        .unwrap()
        .location()
        .set_href("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
        .unwrap()
}

fn main() {
    mount_to_body(move || {
        view! {
            <Router>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/typst_table" view=TypstTable/>
                    // obvious and non-obvious one
                    <Route path="/link" view=Rickroll/>
                    <Route path="/secret/6a892a61d2cca1794717f1413d39e43f" view=Rickroll/>
                </Routes>
            </Router>
        }
    });
}
