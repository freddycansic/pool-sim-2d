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
        let ball_collisions = engine::ball_collisions(&mut balls);

        engine::resolve_wall_collisions(&mut balls, get_frame_time());
        engine::resolve_ball_collisions(&mut balls, &ball_collisions);

        table::render();
        balls.iter().for_each(ball::Ball::render);

        next_frame().await
    }
}
