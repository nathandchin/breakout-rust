use raylib::prelude::*;

#[derive(Debug)]
pub struct Ball {
    pos: Vector2,
    prev_pos: Vector2,
    radius: f32,
    velocity: Vector2,
}

#[derive(Debug)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            prev_pos: Vector2::new(x, y),
            radius: 15.0,
            velocity: Vector2::new(0.1, -0.1),
        }
    }

    fn get_rect_collision_sider(&self, rec: &Rectangle) -> Option<Side> {
        if !rec.check_collision_circle_rec(self.pos, self.radius) {
            return None;
        }
        if self.prev_pos.x < rec.x {
            return Some(Side::Left);
        }
        if self.prev_pos.x > rec.x + rec.width {
            return Some(Side::Right);
        }
        if self.prev_pos.y < rec.y {
            return Some(Side::Top);
        }
        if self.prev_pos.y > rec.y + rec.height {
            return Some(Side::Bottom);
        }
        None
    }

    pub fn update(&mut self, recs: &Vec<Rectangle>) -> Option<(usize, Side)> {
        let mut rv = None;
        for (i, rec) in recs.iter().enumerate() {
            if let Some(side) = self.get_rect_collision_sider(rec) {
                match side {
                    Side::Top | Side::Bottom => self.velocity *= Vector2::new(1.0, -1.0),
                    Side::Left | Side::Right => self.velocity *= Vector2::new(-1.0, 1.0),
                }
                rv = Some((i, side));
            }
        }

        self.prev_pos = self.pos;
        self.pos += self.velocity;
        rv
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
