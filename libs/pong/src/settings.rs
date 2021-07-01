pub const GAME_TITLE: &str = "Crappy Pong";

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

pub const X_OFFSET: f32 = 20.0; // distance from each paddle to their respective walls
pub const PADDLE_WIDTH: f32 = 12.0;
pub const PADDLE_HEIGHT: f32 = 75.0;
pub const PADDLE_SPEED: f32 = 8.0;

pub const BALL_RADIUS: f32 = 10.0;
pub const BALL_MIN_VEL: f32 = 2.0;
pub const BALL_MAX_VEL: f32 = 3.0;
pub const BALL_MAX_BOUNCE_ANGLE: f32 = 75.0; // Max angle in radians at which a ball can bounce off a paddle
pub const BALL_ACCELERATION: f32 = 1.0;

use crate::game::mode::*;
use crate::players::*;
pub const PLAYER_VS_PLAYER: Mode = Mode::TwoPlayer(Player::Human, Player::Human);
pub const PLAYER_VS_AI: Mode = Mode::TwoPlayer(Player::Human, Player::Computer);
pub const PLAYER_VS_SELF: Mode = Mode::OnePlayer(Player::Human);
pub const AI_VS_PLAYER: Mode = Mode::TwoPlayer(Player::Computer, Player::Human);
pub const AI_VS_AI: Mode = Mode::TwoPlayer(Player::Computer, Player::Computer);
pub const AI_VS_SELF: Mode = Mode::OnePlayer(Player::Computer);
pub const TRAIN_AI: Mode = Mode::TrainAi(Player::Computer);

#[derive(Clone, Debug)]
pub struct Config {
    pub eye_photoreceptors: usize,
    pub brain_neurons: usize,
    pub outputs: usize,
    pub generation_length: usize,
    pub population_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            eye_photoreceptors: 5, // Paddle Y, Ball X, Ball Y, Ball VX, Ball VY
            brain_neurons: 15,
            outputs: 1,            // Whether the move the paddle up or down
            generation_length: 10, // How many serves to play for
            population_size: 100,  // How big should the pool of trainee AIs be
        }
    }
}
