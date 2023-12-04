use leptos::*;

#[derive(Default, Clone, Copy, Debug)]
enum CellType {
    #[default]
    SquareBrackets,
    Dollars,
}

fn convert_line(line: &str, select: CellType) -> String {
    let mut out = "  ".to_string();
    if let CellType::SquareBrackets = select {
        line.split('\t').for_each(|elem| {
            out.push('[');
            out.push_str(elem);
            out.push_str("], ");
        });
    } else {
        line.split('\t').for_each(|elem| {
            out.push('$');
            out.push_str(elem);
            out.push_str("$, ");
        });
    }
    out.push('\n');
    out
}

fn convert(table: String, select: CellType) -> String {
    let table = table.replace(',', ".");
    let columns = table
        .lines()
        .next()
        .map(|line| line.split('\t').count())
        .unwrap_or(0);

    let cells = table
        .lines()
        .map(|l| convert_line(l, select))
        .collect::<String>();
    let output = format!(
        r#"#table(
  columns: {columns},
{cells})"#
    );
    output
}

#[component]
fn Cell(select: WriteSignal<CellType>) -> impl IntoView {
    view! {
        <p>How to insert data into table cells:</p>

        <input type="radio" name="celltype" id="brackets" value="brackets" checked="checked"
            on:input=move |_| {select(CellType::SquareBrackets)}/>
        <label for="brackets">square brackets (regular text)</label>

        <input type="radio" name="celltype" id="dollars" value="dollars"
            on:input=move |_| {select(CellType::Dollars)}/>
        <label for="dollars">dollars (equation)</label>
    }
}

#[component]
fn TextField(sink: WriteSignal<String>) -> impl IntoView {
    view! {
        <textarea
        placeholder="Input: "
        on:input=move |ev| {
            sink(event_target_value(&ev));
        }
        />
    }
}

#[component]
fn OutputField(source: ReadSignal<String>, select: ReadSignal<CellType>) -> impl IntoView {
    view! {
        <textarea placeholder="Output: " readonly=true >
        {move || {
            let output = convert(source(), select());
            output
            }}
        </textarea>
    }
}

#[component]
pub fn typst_table() -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (select, set_select) = create_signal(CellType::default());

    view! {
        <h2>"Insert a Ctrl-C selection from your favorite spreadsheet application,
            and this thing will convert it to a Typst table."
        </h2>

        <Cell select=set_select />
        <TextField sink=set_input/>
        <OutputField source=input select=select/>
    }
}
