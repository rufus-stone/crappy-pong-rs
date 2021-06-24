use ggez::graphics::Rect;

use crate::core::Ball;
use crate::settings::Config;
use crate::settings::*;

#[derive(Clone, Debug)]
pub struct Eye {
    pub(crate) photoreceptors: Vec<f32>,
}

impl Eye {
    pub fn new(config: &Config) -> Self {
        Self {
            photoreceptors: vec![-1.0; config.eye_photoreceptors],
        }
    }

    pub fn from_vision(vision: &[f32]) -> Self {
        //log::warn!("New eye from vision: {:?}", vision);
        Self {
            photoreceptors: vision.to_owned(),
        }
    }

    pub fn step(&self, config: &Config, paddle: Rect, ball: &Ball) -> Self {
        let mut vision: Vec<f32> = vec![0.0; config.eye_photoreceptors];

        // Our 5 eye_photoreceptors are: Paddle Y, Ball X, Ball Y, Ball VX, Ball VY
        vision[0] = paddle.center().y / (SCREEN_HEIGHT - paddle.h);
        vision[1] = ball.rect.center().x / (SCREEN_WIDTH - ball.rect.w);
        vision[2] = ball.rect.center().y / (SCREEN_HEIGHT - ball.rect.h);
        vision[3] = ball.vel.x;
        vision[4] = ball.vel.y;

        log::info!("vision: {:?}", &vision);

        Self::from_vision(&vision)
    }
}
