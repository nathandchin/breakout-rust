use raylib::prelude::*;

pub struct Paddle {
    pub rect: Rectangle,
}

impl Paddle {
    pub fn new(x: f32, y: f32,width: f32, height: f32) -> Self {
        Self {
            rect: Rectangle {
                x,
                y,
                width,
                height,
            },
        }
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        let mouse_x = d.get_mouse_x();
        self.rect.x = mouse_x as f32 - self.rect.width / 2.0;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.rect, Color::WHITE);
    }
}
