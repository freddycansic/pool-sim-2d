use itertools::Itertools;

use crate::ball::Ball;

pub fn simulate(balls: &mut [Ball], deltatime: f32) {
    balls.iter_mut().for_each(|ball| ball.step(deltatime))
}

pub fn collisions<'a>(balls: &'a mut [Ball]) -> Vec<(&'a mut Ball, &'a mut Ball)> {
    balls
        .iter_mut()
        .combinations(2)
        .filter(|pair| pair[0].colliding(pair[1]))
        .map(|pair| (pair[0], pair[1]))
        .collect_vec()
}

pub fn resolve(collisions: &[(&mut Ball, &mut Ball)]) {
    for (ball, other) in collisions {
        ball.velocity *= -1.0;
        other.velocity *= -1.0;
    }
}
