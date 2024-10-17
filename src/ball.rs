use cgmath::{MetricSpace, Vector2, Zero};
use macroquad::color::Color;
use macroquad::prelude::*;

use crate::{SCALE, table};

// 1 inch radius
const BALL_RADIUS: f32 = 0.083333 * SCALE;
const CUE_RADIUS: f32 = 0.072916 * SCALE;

#[derive(Debug, Clone)]
pub struct Ball {
    pub color: Color,
    pub position: Vector2<f32>,
    pub radius: f32,
    pub velocity: Vector2<f32>,
}

impl Ball {
    pub fn new(position: Vector2<f32>, velocity: Vector2<f32>, radius: f32, color: Color) -> Self {
        Ball {
            position,
            radius,
            color,
            velocity,
        }
    }

    pub fn new_ball(position: Vector2<f32>, color: Color) -> Self {
        Self::new(position, Vector2::zero(), BALL_RADIUS, color)
    }

    pub fn new_cue_ball(position: Vector2<f32>) -> Self {
        Self::new(position, Vector2::new(0.0001 * SCALE, 0.0), CUE_RADIUS, WHITE)
    }

    pub fn colliding(&self, other: &Ball) -> bool {
        self.position.distance2(other.position) < (self.radius + other.radius).powf(2.0)
    }

    pub fn step(&mut self, deltatime: f32) {
        self.position += self.velocity * deltatime;
        // self.velocity *= 0.99;
    }

    pub fn render(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}

pub fn get_starting_balls() -> Vec<Ball> {
    vec![
        Ball::new_cue_ball(Vector2::new(
            table::x() + table::LENGTH / 4.0,
            screen_height() / 2.0,
        )),
        // TODO
        Ball::new_ball(
            Vector2::new(
                table::x() + table::LENGTH * 3.0 / 4.0,
                screen_height() / 2.0 + 1.0,
            ),
            YELLOW,
        ),
    ]
}
