use ggez::event;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard;
use ggez::mint::*;
use ggez::{Context, GameResult};
use glam::*;
use rand::RngCore;

use crate::ai::player::*;
use crate::cli;
use crate::gui;
use crate::player::*;
use crate::settings::*;

#[derive(Debug)]
pub enum Wall {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug)]
pub enum Paddle {
    Left,
    Right,
}

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
                Mode::OnePlayer(_) | Mode::TrainAI(_) => Rect::new(
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
                Mode::TrainAI(_) => {
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
    fn ball_hit_paddle(&mut self) -> Option<Paddle> {
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
    fn ball_hit_wall(&mut self) -> Option<Wall> {
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
    fn handle_input(&mut self, ctx: &mut Context) {
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

    fn reset_paddles(&mut self) {
        self.paddle_left = Rect::new(
            X_OFFSET,
            SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );

        self.paddle_right = match &self.mode {
            Mode::OnePlayer(_) | Mode::TrainAI(_) => Rect::new(
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

/// Move the specified paddle, but prevent it from moving off the screen
fn move_paddle(paddle: &mut Rect, amount: f32) {
    if paddle.top() + amount < 0.0 {
        paddle.y = 0.0;
    } else if paddle.bottom() + amount > SCREEN_HEIGHT {
        paddle.y = SCREEN_HEIGHT - PADDLE_HEIGHT;
    } else {
        paddle.y += amount;
    }
}

impl event::EventHandler for GameState {
    /// Called every frame
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.mode {
            // For regular one or two player games we'll draw stuff properly etc
            Mode::OnePlayer(_) | Mode::TwoPlayer(_, _) => {
                let desired_fps = cli::get_target_fps() as u32;
                //const DESIRED_FPS: u32 = 144;

                while ggez::timer::check_update_time(ctx, desired_fps) {
                    // Only handle key presses if the game isn't paused
                    match self.pause_for {
                        0 => {
                            // Handle player and/or AI control input
                            self.handle_input(ctx);

                            // Move the ball based on its velocity
                            self.ball.rect.translate(self.ball.vel);

                            // Check for ball-on-paddle collisions and reverse horizontal velocity of the ball (and increase it slightly!)
                            // This will score a point for P1 if it's a 1 player game
                            if let Some(paddle) = self.ball_hit_paddle() {
                                log::warn!("{:?} paddle hit!", paddle);

                                // Reverse the direction and slightly increase the horizontal speed
                                self.ball.vel.x *= -1.01;
                            }

                            // Check for ball-on-wall collisions act accordingly
                            match self.ball_hit_wall() {
                                // If it hit the top or bottom wall, just reverse the vertical velocity of the ball (and increase it slightly!)
                                Some(Wall::Top) | Some(Wall::Bottom) => {
                                    log::warn!("Top/Bottom wall hit!");
                                    self.ball.bounce_off(Wall::Top); // This is a little clumsy, but a Wall::Top or Wall::Bottom will do the same thing
                                }

                                // If it hit the left wall, either score a point for P2 in a 2 player game, or dock a point from P1 in a 1 player game
                                Some(Wall::Left) => {
                                    log::warn!("Left wall hit!");

                                    match self.mode {
                                        Mode::OnePlayer(_) | Mode::TrainAI(_) => {
                                            self.score.p1 -= 1;

                                            // Pause for 1 second's worth of frames before starting over
                                            self.pause_for = ggez::timer::fps(ctx) as u64;
                                        }
                                        Mode::TwoPlayer(_, _) => {
                                            self.score.p2 += 1;

                                            // Pause for 1 second's worth of frames before starting over
                                            self.pause_for = ggez::timer::fps(ctx) as u64;
                                        }
                                    }
                                }

                                // If it hit the right wall, score a point for P1 in a 2 player game, otherwise do nothing (in a 1 player game this should never happen)
                                Some(Wall::Right) => {
                                    log::warn!("Right wall hit!");
                                    if let Mode::TwoPlayer(_, _) = self.mode {
                                        self.score.p1 += 1;

                                        // Pause for 1 second's worth of frames before starting over
                                        self.pause_for = ggez::timer::fps(ctx) as u64;
                                    }
                                }

                                None => {}
                            }
                        }
                        1 => {
                            self.pause_for -= 1;

                            // Reset the ball
                            self.ball = Ball::random();

                            // Reset the paddles
                            self.reset_paddles();
                        }
                        _ => self.pause_for -= 1,
                    }
                }
            }
            // Don't bother drawing etc for AI training modes
            Mode::TrainAI(_) => {
                // Handle AI control input
                self.handle_input(ctx);

                // Move the ball based on its velocity
                self.ball.rect.translate(self.ball.vel);

                // Check for ball-on-paddle collisions and reverse horizontal velocity of the ball (and increase it slightly!)
                if let Some(paddle) = self.ball_hit_paddle() {
                    log::warn!("{:?} paddle hit!", paddle);

                    // Reverse the direction and slightly increase the horizontal speed
                    self.ball.vel.x *= -1.01;
                }

                // Check for ball-on-wall collisions act accordingly
                match self.ball_hit_wall() {
                    // If it hit the top or bottom wall, just reverse the vertical velocity of the ball (and increase it slightly!)
                    Some(Wall::Top) | Some(Wall::Bottom) => {
                        log::warn!("Top/Bottom wall hit!");
                        self.ball.bounce_off(Wall::Top); // This is a little clumsy, but a Wall::Top or Wall::Bottom will do the same thing
                    }

                    // If the ball hit the left wall, stop the sim and return the score
                    Some(Wall::Left) => {
                        log::warn!("Left wall hit!");

                        self.score.p1 -= 1;

                        // Reset the ball
                        self.ball = Ball::random();

                        // Reset the paddles
                        self.reset_paddles();
                    }

                    // In a 1 player game the ball should never hit the right wall, so don't bother handling that
                    Some(_) => {}

                    // Do nothing if the ball didn't hit a wall
                    None => {}
                }
            }
        }

        Ok(())
    }

    /// Draw the game screen
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match &self.mode {
            Mode::OnePlayer(_) | Mode::TwoPlayer(_, _) => self.render_game(ctx), // Only render the screen for proper games
            Mode::TrainAI(_) => self.render_training(ctx),
        }
    }
}

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

    fn random() -> Ball {
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

    fn bounce_off(&mut self, wall: Wall) {
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

#[derive(Debug, Default)]
pub struct Score {
    pub p1: i16,
    pub p2: i16,
}
