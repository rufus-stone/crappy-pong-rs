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

use crate::player::*;
pub const PLAYER_VS_PLAYER: Mode = Mode::TwoPlayer(Player::Human, Player::Human);
pub const PLAYER_VS_AI: Mode = Mode::TwoPlayer(Player::Human, Player::Computer);
pub const PLAYER_VS_SELF: Mode = Mode::OnePlayer(Player::Human);
pub const AI_VS_PLAYER: Mode = Mode::TwoPlayer(Player::Computer, Player::Human);
pub const AI_VS_AI: Mode = Mode::TwoPlayer(Player::Computer, Player::Computer);
pub const AI_VS_SELF: Mode = Mode::OnePlayer(Player::Computer);
