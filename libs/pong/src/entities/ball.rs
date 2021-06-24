use ggez::graphics::*;
use ggez::mint::*;

use crate::settings::*;

use super::wall::*;

#[derive(Debug, Clone)]
pub struct Ball {
    pub(crate) rect: Rect,
    pub(crate) vel: Vector2<f32>,
    pub(crate) spd: f32,
}

impl Ball {
    /*fn new(x: f32, y: f32, radius: f32) -> Ball {
        Ball {
            rect: Rect::new(x, y, radius, radius),
            vel: Vector2::<f32> { x: 0.0, y: 0.0 }, //vel: Vector2::<f32> { x: 1.0, y: -0.5 },
            spd: 0.0,
        }
    }*/

    pub fn random() -> Ball {
        log::warn!("New ball");
        use rand::prelude::*;

        let mut prng = rand::thread_rng();

        let mut random_velocity = || -> f32 {
            let flip = prng.gen::<bool>();

            match flip {
                true => -prng.gen_range(BALL_MIN_VEL..=BALL_MAX_VEL),
                false => prng.gen_range(BALL_MIN_VEL..=BALL_MAX_VEL),
            }
        };

        let vel_x = random_velocity();
        let vel_y = random_velocity();
        let spd: f32 = vel_x.hypot(vel_y);

        Ball {
            rect: Rect::new(
                SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                BALL_RADIUS,
                BALL_RADIUS,
            ),
            vel: Vector2::<f32> { x: vel_x, y: vel_y },
            spd,
        }
    }

    pub fn bounce_off(&mut self, wall: Wall) {
        match wall {
            Wall::Top | Wall::Bottom => {
                if self.vel.y > 0.0 {
                    log::info!(
                        "pos - bvy: {}, bvy * BALL_ACCELERATION: {}, clamped: {}, r: {}",
                        self.vel.y,
                        self.vel.y * BALL_ACCELERATION,
                        (self.vel.y * BALL_ACCELERATION).clamp(BALL_MIN_VEL, BALL_MAX_VEL),
                        -(self.vel.y * BALL_ACCELERATION).clamp(BALL_MIN_VEL, BALL_MAX_VEL)
                    );
                    self.vel.y =
                        (self.vel.y * -BALL_ACCELERATION).clamp(-BALL_MAX_VEL, -BALL_MIN_VEL);
                } else {
                    log::info!(
                        "neg - bvy: {}, bvy * BALL_ACCELERATION: {}, clamped: {}, r: {}",
                        self.vel.y,
                        self.vel.y * BALL_ACCELERATION,
                        (self.vel.y * BALL_ACCELERATION).clamp(BALL_MIN_VEL, BALL_MAX_VEL),
                        -(self.vel.y * BALL_ACCELERATION).clamp(BALL_MIN_VEL, BALL_MAX_VEL)
                    );
                    self.vel.y = (self.vel.y * BALL_ACCELERATION).clamp(BALL_MIN_VEL, BALL_MAX_VEL);
                }
            }
            _ => {}
        }
    }
}
