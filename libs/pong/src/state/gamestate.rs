use ggez::graphics::Rect;
use ggez::input::keyboard;
use ggez::{Context, GameResult};
use rand::RngCore;

use crate::entities::{ball::*, paddle::*, wall::*};
use crate::game::engine::*;
use crate::game::mode::Mode;
use crate::players::ai::player::*;
use crate::players::human::player::*;
use crate::players::Move;
use crate::players::*;
use crate::settings::*;

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

impl GameState {
    /// Create a new GameState struct for a game with the specified number of players
    pub fn new(mode: Mode, prng: &mut dyn RngCore) -> GameResult<GameState> {
        Ok(GameState {
            paddle_left: Rect::new(
                X_OFFSET,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),

            paddle_right: match &mode {
                Mode::OnePlayer(_) | Mode::TrainAi(_) => Rect::new(
                    SCREEN_WIDTH - X_OFFSET - PADDLE_WIDTH,
                    0.0,
                    PADDLE_WIDTH,
                    SCREEN_HEIGHT,
                ),
                Mode::TwoPlayer(_, _) => Rect::new(
                    SCREEN_WIDTH - X_OFFSET - PADDLE_WIDTH,
                    SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                    PADDLE_WIDTH,
                    PADDLE_HEIGHT,
                ),
            },
            ball: Ball::random(),
            score: Score::default(),
            pause_for: 0,
            player_one: match &mode {
                Mode::OnePlayer(p1) => match p1 {
                    Player::Human => {
                        log::warn!("P1: Human");
                        Box::new(HumanPlayer::new(keyboard::KeyCode::W, keyboard::KeyCode::S))
                    }
                    Player::Computer => {
                        log::warn!("P1: AI");
                        Box::new(AiPlayer::random(&Config::default(), prng))
                    }
                },
                Mode::TwoPlayer(p1, _) => match p1 {
                    Player::Human => {
                        log::warn!("P1: Human vs...");
                        Box::new(HumanPlayer::new(keyboard::KeyCode::W, keyboard::KeyCode::S))
                    }
                    Player::Computer => {
                        log::warn!("P1: AI vs...");
                        Box::new(AiPlayer::random(&Config::default(), prng))
                    }
                },
                Mode::TrainAi(_) => {
                    log::warn!("P1: AI training");
                    Box::new(AiPlayer::random(&Config::default(), prng))
                }
            },
            player_two: match &mode {
                Mode::TwoPlayer(_, p2) => match p2 {
                    Player::Human => {
                        log::warn!("... P2: Human");
                        Some(Box::new(HumanPlayer::new(
                            keyboard::KeyCode::Up,
                            keyboard::KeyCode::Down,
                        )))
                    }
                    Player::Computer => {
                        log::warn!("... P2: AI");
                        Some(Box::new(AiPlayer::random(&Config::default(), prng)))
                    }
                },
                _ => None,
            },
            mode,
        })
    }

    /// Check if the ball hit a paddle
    pub fn ball_hit_paddle(&mut self) -> Option<Paddle> {
        if self.ball.vel.x < 0.0 && self.ball.rect.overlaps(&self.paddle_left) {
            // In 1 player mode we also score a point
            if let Mode::OnePlayer(_) = self.mode {
                self.score.p1 += 1;
            }

            Some(Paddle::Left)
        } else if self.ball.vel.x > 0.0 && self.ball.rect.overlaps(&self.paddle_right) {
            Some(Paddle::Right)
        } else {
            None
        }
    }

    /// Check if the ball hit a wall
    pub fn ball_hit_wall(&mut self) -> Option<Wall> {
        if self.ball.vel.y < 0.0 && self.ball.rect.top() < 0.0 {
            Some(Wall::Top)
        } else if self.ball.vel.y > 0.0 && self.ball.rect.bottom() > SCREEN_HEIGHT - BALL_RADIUS {
            Some(Wall::Bottom)
        } else if self.ball.rect.left() < 0.0 {
            Some(Wall::Left)
        } else if self.ball.rect.right() > SCREEN_WIDTH - BALL_RADIUS {
            Some(Wall::Right)
        } else {
            None
        }
    }

    /// Checks for Human and/or AI player input and moves the paddles accordingly
    pub fn handle_input(&mut self, ctx: &mut Context) {
        // Check player 1 input
        let p1_move = self
            .player_one
            .make_move(ctx, &Snapshot::new(&self.paddle_left, &self.ball));
        move_paddle(&mut self.paddle_left, p1_move);

        // Check player 2 input, but only if we're playing a 2 player game
        if let Some(player_two) = &self.player_two {
            let p2_move = player_two.make_move(ctx, &Snapshot::new(&self.paddle_right, &self.ball));
            move_paddle(&mut self.paddle_right, p2_move);
        }
    }

    pub fn reset_paddles(&mut self) {
        self.paddle_left = Rect::new(
            X_OFFSET,
            SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );

        self.paddle_right = match &self.mode {
            Mode::OnePlayer(_) | Mode::TrainAi(_) => Rect::new(
                SCREEN_WIDTH - X_OFFSET - PADDLE_WIDTH,
                0.0,
                PADDLE_WIDTH,
                SCREEN_HEIGHT,
            ),
            Mode::TwoPlayer(_, _) => Rect::new(
                SCREEN_WIDTH - X_OFFSET - PADDLE_WIDTH,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
        };
    }
}
