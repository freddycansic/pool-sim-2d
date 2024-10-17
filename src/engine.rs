use cgmath::InnerSpace;
use itertools::Itertools;

use crate::ball::Ball;
use crate::table;

pub fn simulate(balls: &mut [Ball], deltatime: f32) {
    balls.iter_mut().for_each(|ball| ball.step(deltatime))
}

pub fn ball_collisions(balls: &[Ball]) -> Vec<(usize, usize)> {
    (0..balls.len())
        .combinations(2)
        .filter(|pair| balls[pair[0]].colliding(&balls[pair[1]]))
        .map(|pair| (pair[0], pair[1]))
        .collect_vec()
}

pub fn resolve_wall_collisions(balls: &mut [Ball], deltatime: f32) {
    for ball in balls.iter_mut() {
        if ball.position.x + ball.radius > table::x() + table::LENGTH
            || ball.position.x - ball.radius < table::x()
        {
            ball.velocity.x *= -table::WALL_BITE;
            ball.step(deltatime);
        }

        if ball.position.y + ball.radius > table::y() + table::WIDTH
            || ball.position.y - ball.radius < table::y()
        {
            ball.velocity.y *= -table::WALL_BITE;
            ball.step(deltatime);
        }
    }
}

pub fn resolve_ball_collisions(balls: &mut [Ball], collisions: &[(usize, usize)]) {
    for (x, y) in collisions {
        let relative_velocity = balls[*x].velocity - balls[*y].velocity;
        let collision_normal = (balls[*y].position - balls[*x].position).normalize();
        let relative_velocity_normal = relative_velocity.normalize();

        let alignment = relative_velocity_normal.dot(collision_normal);

        if alignment > 0.0 {
            let transfer_amount = alignment * relative_velocity.magnitude();

            balls[*x].velocity -= collision_normal * transfer_amount;
            balls[*y].velocity += collision_normal * transfer_amount;
        }
    }
}
