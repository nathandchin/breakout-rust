use raylib::prelude::*;

pub struct Ball {
    pos: Vector2,
    radius: f32,
    velocity: Vector2,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            radius: 15.0,
            velocity: Vector2::new(0.05, -0.05),
        }
    }

    pub fn update(&mut self) {
        self.pos += self.velocity;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(
            self.pos.x as i32,
            self.pos.y as i32,
            self.radius,
            Color::WHITE,
        );
    }
}
