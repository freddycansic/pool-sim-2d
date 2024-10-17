use macroquad::prelude::*;

mod ball;
mod engine;
mod table;

const SCALE: f32 = 100.0;

#[macroquad::main("pool-sim-2d")]
async fn main() {
    let mut balls = ball::get_starting_balls();

    loop {
        clear_background(WHITE);

        engine::simulate(&mut balls, get_frame_time());
        let collisions = engine::collisions(&mut balls);
        engine::resolve(&mut balls, &collisions);

        table::render();
        balls.iter().for_each(ball::Ball::render);

        next_frame().await
    }
}
