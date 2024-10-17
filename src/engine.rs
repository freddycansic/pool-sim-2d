use itertools::Itertools;

use crate::ball::Ball;

pub fn simulate(balls: &mut [Ball], deltatime: f32) {
    balls.iter_mut().for_each(|ball| ball.step(deltatime))
}

pub fn collisions(balls: &[Ball]) -> Vec<(usize, usize)> {
    (0..balls.len())
        .combinations(2)
        .filter(|pair| balls[pair[0]].colliding(&balls[pair[1]]))
        .map(|pair| (pair[0], pair[1]))
        .collect_vec()
}

pub fn resolve(balls: &mut [Ball], collisions: &[(usize, usize)]) {
    for (ball_index, other_index) in collisions {
        balls[*ball_index].velocity *= -1.0;
        balls[*other_index].velocity *= -1.0;
    }
}
