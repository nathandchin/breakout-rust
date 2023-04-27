use raylib::prelude::*;

mod paddle;
use paddle::Paddle;
mod ball;
use ball::Ball;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 1600;

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Breakout").build();
    let mut p = Paddle::new(
        WIDTH as f32 / 8.0,
        HEIGHT as f32 / 40.0,
        WIDTH as f32 / 2.0,
        HEIGHT as f32 * (4.0 / 5.0),
    );
    let mut b = Ball::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
    let walls = vec![
        Rectangle::new(0.0, 0.0, WIDTH as f32, 0.0), // Top
        Rectangle::new(0.0, HEIGHT as f32, WIDTH as f32, 0.0), // Bottom
        Rectangle::new(0.0, 0.0, 0.0, HEIGHT as f32), // Left
        Rectangle::new(WIDTH as f32, 0.0, 0.0, HEIGHT as f32), // Right
    ];

    while !rl.window_should_close() {
        let mut colliders = walls.clone();
        colliders.push(p.rect.clone());

        let mut d = rl.begin_drawing(&thread);

        // dbg!(&walls);

        p.update(&mut d);
        b.update(&colliders);

        d.clear_background(Color::BLACK);
        p.draw(&mut d);
        b.draw(&mut d);
    }
}
