use std::time::Duration;

use leptos::*;

const PIXEL_SIZE_PX: usize = 20;
const TILE_SIZE_PX: usize = PIXEL_SIZE_PX + 6 / 5;

#[derive(Clone)]
struct Field {
    x: usize,
    y: usize,
    content: Vec<Vec<bool>>,
}

impl Field {
    fn new_random(x: usize, y: usize) -> Field {
        let mut content = vec![vec![true; x]; y];
        content
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|pixel| *pixel = rand::random()));
        Field { content, x, y }
    }
    fn new(x: usize, y: usize) -> Field {
        Field {
            content: vec![vec![true; x]; y],
            x,
            y,
        }
    }

    // returns the value of a cell from field as `u8`,
    // alive cell yields one, dead cell yield zero, cell outside the
    // region also yields zero
    fn clamped_read(&self, x: usize, y: usize) -> u8 {
        if let Some(Some(cell)) = self.content.get(y).map(|row| row.get(x)) {
            *cell as u8
        } else {
            0
        }
    }

    fn will_be_alive(&self, x: usize, y: usize) -> bool {
        let alive_neigbors = (-1..=1)
            .map(|y_offset: isize| {
                (-1..=1)
                    .map(|x_offset: isize| {
                        if (x, y) != (0, 0) {
                            self.clamped_read(
                                (x as isize + x_offset) as usize,
                                (y as isize + y_offset) as usize,
                            )
                        } else {
                            0
                        }
                    })
                    .sum::<u8>()
            })
            .sum::<u8>();

        match (self.content[y][x], alive_neigbors) {
            (true, neigh) if neigh < 2 => false,
            (true, neigh) if neigh == 2 || neigh == 3 => true,
            (true, neigh) if neigh > 3 => false,
            (false, neigh) if neigh == 3 => true,
            (prev_state, _) => prev_state,
        }
    }

    fn update(&mut self) {
        let mut new_field = Field::new(self.x, self.y);

        for (y, row) in new_field.content.iter_mut().enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = self.will_be_alive(x, y);
            }
        }

        self.content = new_field.content;
    }
}

fn draw_square(x: usize, y: usize, is_alive: bool) -> View {
    let color = if is_alive { "#fff" } else { "#000" };
    let move_down = (TILE_SIZE_PX) * y;
    let move_right = (TILE_SIZE_PX) * x;
    view! {
        <div
            class="square"
            style=format!("
                background-color: {color};
                margin-top: {move_down}px;
                margin-left: {move_right}px;
            ")
        />
    }
    .into_view()
}

impl IntoView for Field {
    fn into_view(self) -> View {
        let mut pixels = vec![];

        for (y, row) in self.content.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                pixels.push(draw_square(x, y, *pixel));
            }
        }

        Fragment::new(pixels).into_view()
    }
}

#[component]
pub fn settings(run: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div style="position: fixed">
            <button name="Start/Stop" on:click=move |_| {
        run.update(|s| *s = !*s)
        } />
        </div>
    }
}

#[component]
pub fn game_of_life() -> impl IntoView {
    let window = window().window();
    let width = window.inner_width().unwrap().as_f64().unwrap() as usize / (TILE_SIZE_PX);
    let height = window.inner_height().unwrap().as_f64().unwrap() as usize / (TILE_SIZE_PX);
    let (field, update_field) = create_signal(Field::new_random(width, height));

    // periodic updating
    set_interval(
        move || {
            update_field.update(|field| field.update());
        },
        Duration::from_millis(100),
    );

    let style = format!(
        "
            body {{
                background-color: black;
            }}
            .square {{
                position: absolute;
                height: {PIXEL_SIZE_PX}px;
                width: {PIXEL_SIZE_PX}px;
            }} "
    );
    view! {
        <style> {style} </style>
        {move || {field.get()}}
    }
}
