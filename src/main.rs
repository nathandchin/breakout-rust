use raylib::prelude::*;

mod paddle;
use paddle::Paddle;
mod ball;
use ball::Ball;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Breakout").build();
    let mut p = Paddle::new(
        WIDTH as f32 / 8.0,
        HEIGHT as f32 / 40.0,
        WIDTH as f32 / 2.0,
        HEIGHT as f32 * (4.0 / 5.0),
    );
    let mut b = Ball::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        p.update(&mut d);
        b.update();

        d.clear_background(Color::BLACK);
        p.draw(&mut d);
        b.draw(&mut d);
    }
}
