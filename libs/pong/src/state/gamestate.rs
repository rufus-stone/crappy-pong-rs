use ggez::graphics::Rect;

use crate::entities::ball::Ball;
use crate::game::mode::Mode;
use crate::players::Move;

use super::score::Score;

#[derive(Debug)]
pub struct GameState {
    pub(crate) paddle_left: Rect,
    pub(crate) paddle_right: Rect,
    pub(crate) ball: Ball,
    pub(crate) score: Score,
    pub(crate) pause_for: u64,
    pub(crate) mode: Mode,
    pub(crate) player_one: Box<dyn Move>,
    pub(crate) player_two: Option<Box<dyn Move>>,
}
