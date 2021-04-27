use ggez::graphics::Rect;

use crate::core::Ball;
use crate::settings::Config;
use crate::settings::*;

#[derive(Clone, Debug)]
pub struct Eye {
    pub(crate) energies: Vec<f32>,
}

impl Eye {
    pub fn new(config: &Config) -> Self {
        Self {
            energies: vec![-1.0; config.eye_photoreceptors],
        }
    }

    pub fn from_energies(energies: &[f32]) -> Self {
        log::warn!("New eye from energies: {:?}", energies);
        Self {
            energies: energies.to_owned(),
        }
    }

    pub fn step(&self, config: &Config, paddle: Rect, ball: &Ball) -> Self {
        let mut energies: Vec<f32> = vec![0.0; config.eye_photoreceptors];

        // Our 5 eye_photoreceptors are: Paddle Y, Ball X, Ball Y, Ball VX, Ball VY
        energies[0] = paddle.center().y / (SCREEN_HEIGHT - paddle.h);
        energies[1] = ball.rect.center().x / (SCREEN_WIDTH - ball.rect.w);
        energies[2] = ball.rect.center().y / (SCREEN_HEIGHT - ball.rect.h);
        energies[3] = ball.vel.x;
        energies[4] = ball.vel.y;

        Self::from_energies(&energies)
    }
}
