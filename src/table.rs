use macroquad::prelude::*;

use crate::SCALE;

pub const LENGTH: f32 = 6.0 * SCALE;
pub const WIDTH: f32 = 3.0 * SCALE;
pub const COLOR: Color = BROWN;
pub const WALL_WIDTH: f32 = 0.3 * SCALE;
pub const WALL_BITE: f32 = 0.98;
pub const SLIDING_FRICTION_COEFFICIENT: f32 = 0.25;
pub const ROLLING_FRICTION_COEFFICIENT: f32 = 0.01;

pub fn x() -> f32 {
    screen_width() / 2.0 - LENGTH / 2.0
}

pub fn y() -> f32 {
    screen_height() / 2.0 - WIDTH / 2.0
}

pub fn render() {
    draw_rectangle(
        x() - WALL_WIDTH,
        y() - WALL_WIDTH,
        LENGTH + 2.0 * WALL_WIDTH,
        WALL_WIDTH,
        COLOR,
    ); // top
    draw_rectangle(x() - WALL_WIDTH, y(), WALL_WIDTH, WIDTH, COLOR); // left
    draw_rectangle(
        x() - WALL_WIDTH,
        screen_height() - y(),
        LENGTH + 2.0 * WALL_WIDTH,
        WALL_WIDTH,
        COLOR,
    ); // bottom
    draw_rectangle(x() + LENGTH, y(), WALL_WIDTH, WIDTH, COLOR); // right

    draw_rectangle(x(), y(), LENGTH, WIDTH, DARKGREEN);
}
