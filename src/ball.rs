use cgmath::{InnerSpace, MetricSpace, Vector2, Zero};
use cgmath::num_traits::FloatConst;
use macroquad::color::Color;
use macroquad::prelude::*;

use crate::{SCALE, GRAVITY, table};

// 1 inch radius
const BALL_RADIUS: f32 = 0.083333 * SCALE;
const CUE_RADIUS: f32 = 0.072916 * SCALE;
pub const BALL_RESTITUTION: f32 = 0.95;
pub const BALL_MASS: f32 = 0.096; // 96 grams

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
        Self::new(position, Vector2::zero(), CUE_RADIUS, WHITE)
    }
    
    pub fn mouse_over(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();

        (self.position.x - mouse_x).powf(2.0) + (self.position.y - mouse_y).powf(2.0) < self.radius.powf(2.0)
    }

    pub fn colliding(&self, other: &Ball) -> bool {
        self.position.distance2(other.position) < (self.radius + other.radius).powf(2.0)
    }

    pub fn step(&mut self, deltatime: f32) {
        self.position += self.velocity * deltatime;

        if self.velocity.magnitude2() > 0.0 {
            let friction = -self.velocity.normalize() * (table::SLIDING_FRICTION_COEFFICIENT * BALL_MASS * GRAVITY) * deltatime;
            self.velocity += friction;

            // If the velocity and the friction force are aligned then don't apply the friction
            // This stops the ball from staring to move backwards
            if self.velocity.dot(friction) > 0.0 {
                self.velocity = Vector2::zero();
            }
        }
    }

    pub fn render(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.velocity.x,
            self.position.y + self.velocity.y,
            1.0,
            RED,
        );
    }
}

#[rustfmt::skip]
pub fn get_starting_balls() -> Vec<Ball> {
    let offset_x = 2.0 * BALL_RADIUS * f32::FRAC_PI_6().cos();
    let offset_y = 2.0 * BALL_RADIUS * f32::FRAC_PI_6().sin();

    let start_x = table::x() + table::LENGTH * 3.0 / 4.0;
    let start_y = screen_height() / 2.0;

    vec![
        Ball::new_cue_ball(Vector2::new(table::x() + table::LENGTH / 4.0, start_y)),
        Ball::new_ball(Vector2::new(start_x, start_y), YELLOW),
 
        // Bottom side
        Ball::new_ball(Vector2::new(start_x + 1.0 * offset_x, start_y + 1.0 * offset_y), RED),
        Ball::new_ball(Vector2::new(start_x + 2.0 * offset_x, start_y + 2.0 * offset_y), YELLOW),
        Ball::new_ball(Vector2::new(start_x + 3.0 * offset_x, start_y + 3.0 * offset_y), RED),
        Ball::new_ball(Vector2::new(start_x + 4.0 * offset_x, start_y + 4.0 * offset_y), YELLOW),

        // Top side
        Ball::new_ball(Vector2::new(start_x + 1.0 * offset_x, start_y - 1.0 * offset_y), YELLOW),
        Ball::new_ball(Vector2::new(start_x + 2.0 * offset_x, start_y - 2.0 * offset_y), RED),
        Ball::new_ball(Vector2::new(start_x + 3.0 * offset_x, start_y - 3.0 * offset_y), YELLOW),
        Ball::new_ball(Vector2::new(start_x + 4.0 * offset_x, start_y - 4.0 * offset_y), RED),

        // Middle
        Ball::new_ball(Vector2::new(start_x + 2.0 * offset_x, start_y), BLACK),
        Ball::new_ball(Vector2::new(start_x + 3.0 * offset_x, start_y + 1.0 * offset_y), YELLOW),
        Ball::new_ball(Vector2::new(start_x + 3.0 * offset_x, start_y - 1.0 * offset_y), RED),
        Ball::new_ball(Vector2::new(start_x + 4.0 * offset_x, start_y + 2.0 * offset_y), RED),
        Ball::new_ball(Vector2::new(start_x + 4.0 * offset_x, start_y), YELLOW),
        Ball::new_ball(Vector2::new(start_x + 4.0 * offset_x, start_y - 2.0 * offset_y), RED),
    ]
}
