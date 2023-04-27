use raylib::prelude::*;

pub struct Brick {
    pub rect: Rectangle,
}

impl Brick {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: Rectangle {
                x,
                y,
                width,
                height,
            },
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.rect, Color::GREEN);
    }
}

fn generate_row(y: f32, number: u32, spacing: f32, height: f32, screen_w: f32) -> Vec<Brick> {
    let mut result: Vec<Brick> = vec![];
    let brick_width = (screen_w / number as f32) - spacing;
    let mut curr_start = spacing / 2.0;
    for _ in 0..number {
        result.push(Brick::new(curr_start, y, brick_width, height));
        curr_start += brick_width + spacing;
    }
    result
}

pub fn generate_grid(
    rows: u32,
    cols: u32,
    row_spacing: f32,
    col_spacing: f32,
    cell_height: f32,
    screen_w: f32,
) -> Vec<Vec<Brick>> {
    let mut result: Vec<Vec<Brick>> = vec![];
    let mut curr_start = row_spacing / 2.0;
    for _ in 0..rows {
        result.push(generate_row(
            curr_start,
            cols,
            col_spacing,
            cell_height,
            screen_w,
        ));
        curr_start += cell_height + row_spacing;
    }
    result
}
