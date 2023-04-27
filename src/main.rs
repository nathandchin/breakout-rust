use raylib::prelude::*;

mod paddle;
use paddle::Paddle;
mod ball;
use ball::Ball;
mod bricks;
use bricks::{generate_grid, Brick};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1600;

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Breakout").build();
    let mut paddle = Paddle::new(
        WIDTH as f32 / 2.0,
        HEIGHT as f32 * (4.0 / 5.0),
        WIDTH as f32 / 8.0,
        HEIGHT as f32 / 40.0,
    );
    let mut ball = Ball::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
    let walls = vec![
        Rectangle::new(0.0, 0.0, WIDTH as f32, 0.0), // Top
        Rectangle::new(0.0, HEIGHT as f32, WIDTH as f32, 0.0), // Bottom
        Rectangle::new(0.0, 0.0, 0.0, HEIGHT as f32), // Left
        Rectangle::new(WIDTH as f32, 0.0, 0.0, HEIGHT as f32), // Right
    ];
    let bricks: Vec<Brick> = generate_grid(15, 6, 10.0, 10.0, 20.0, WIDTH as f32)
        .into_iter()
        .flatten()
        .collect();

    while !rl.window_should_close() {
        let mut colliders = walls.clone();
        for brick in &bricks {
            colliders.push(brick.rect.clone());
        }
        colliders.push(paddle.rect.clone());

        let mut d = rl.begin_drawing(&thread);

        paddle.update(&mut d);
        ball.update(&colliders);

        d.clear_background(Color::BLACK);
        paddle.draw(&mut d);
        ball.draw(&mut d);
        for brick in &bricks {
            brick.draw(&mut d);
        }
    }
}
