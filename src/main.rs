use cgmath::{InnerSpace, Vector2, Zero};
use macroquad::prelude::*;

use ball::Ball;

mod ball;
mod engine;
mod table;

const SCALE: f32 = 300.0;
const GRAVITY: f32 = 9.81;
const FPS_CAP: f64 = 60.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pool Simulation".to_owned(),
        sample_count: 16,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    next_frame().await; // Allow the window 1 frame to adjust to correct size (tiling window manager)

    let mut balls = ball::get_starting_balls();
    let mut dragging = false;
    let mut drag_end = Vector2::zero();

    let mut last_time = get_time();

    loop {
        let elapsed = get_time() - last_time;
        if elapsed < 1.0 / FPS_CAP {
            continue;
        }

        last_time = get_time();

        clear_background(WHITE);

        engine::simulate(&mut balls, get_frame_time());
        let ball_collisions = engine::ball_collisions(&mut balls);

        engine::resolve_wall_collisions(&mut balls, get_frame_time());
        engine::resolve_ball_collisions(&mut balls, &ball_collisions);

        table::render();
        balls.iter().for_each(Ball::render);

        if !dragging && is_mouse_down_on_cue(&balls[0]) {
            dragging = true;
        }

        if dragging && is_mouse_button_down(MouseButton::Left) {
            drag_end = mouse_position_vector();

            draw_line(
                balls[0].position.x,
                balls[0].position.y,
                drag_end.x,
                drag_end.y,
                1.0,
                BLUE,
            );
        }

        if dragging && is_mouse_button_released(MouseButton::Left) {
            dragging = false;

            let force = -(drag_end - balls[0].position) / SCALE;
            dbg!(force.magnitude());
            balls[0].velocity += force * get_frame_time() * 100.0;
        }
        next_frame().await
    }
}

fn is_mouse_down_on_cue(cue: &Ball) -> bool {
    is_mouse_button_down(MouseButton::Left) && cue.mouse_over()
}

fn mouse_position_vector() -> Vector2<f32> {
    let (mouse_x, mouse_y) = mouse_position();

    Vector2::new(mouse_x, mouse_y)
}
