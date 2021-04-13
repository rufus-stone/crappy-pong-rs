use ggez::event;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard;
use ggez::mint::*;
use ggez::{Context, GameResult};
use glam::*;

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
pub struct GameState {
    paddle_left: Rect,
    paddle_right: Rect,
    ball: Ball,
    score: Score,
    pause_for: u64,
    mode: Mode,
}

impl GameState {
    /// Create a new GameState struct for a game with the specified number of players
    pub fn new(mode: Mode) -> GameResult<GameState> {
        Ok(GameState {
            paddle_left: Rect::new(
                X_OFFSET,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),

            paddle_right: match mode {
                Mode::OnePlayer(_) => Rect::new(
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
            ball: Ball::new(
                SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                BALL_RADIUS,
            ),
            score: Score::default(),
            pause_for: 0,
            mode,
        })
    }

    /// Check if the ball hit a paddle
    fn ball_hit_paddle(&mut self) -> bool {
        if self.ball.vel.x < 0.0 && self.ball.rect.overlaps(&self.paddle_left) {
            // In 1 player mode we also score a point
            if let Mode::OnePlayer(_) = self.mode {
                self.score.p1 += 1;
            }

            true
        } else {
            self.ball.vel.x > 0.0 && self.ball.rect.overlaps(&self.paddle_right)
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

    fn handle_player_1_input(&mut self, ctx: &mut Context) {
        // Check for key presses and move Player 1 paddle accordingly
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            move_paddle(&mut self.paddle_left, -PADDLE_SPEED);
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S) {
            move_paddle(&mut self.paddle_left, PADDLE_SPEED);
        }
    }

    fn handle_player_2_input(&mut self, ctx: &mut Context) {
        // Check for key presses and move Player 2 paddle accordingly
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Up) {
            move_paddle(&mut self.paddle_right, -PADDLE_SPEED);
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Down) {
            move_paddle(&mut self.paddle_right, PADDLE_SPEED);
        }
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
        // Only handle key presses if the game isn't paused
        match self.pause_for {
            0 => {
                // TODO: Change this to something more sensible!!!
                match &self.mode {
                    Mode::OnePlayer(p1) => match p1 {
                        Player::Human => self.handle_player_1_input(ctx),
                        Player::Computer => {}
                    },
                    Mode::TwoPlayer(p1, p2) => match (p1, p2) {
                        (Player::Human, Player::Human) => {
                            self.handle_player_1_input(ctx);
                            self.handle_player_2_input(ctx);
                        }
                        (Player::Human, Player::Computer) => {}
                        (Player::Computer, Player::Human) => {}
                        (Player::Computer, Player::Computer) => {}
                    },
                }

                // Move the ball based on its velocity
                self.ball.rect.translate(self.ball.vel);

                // Check for ball-on-paddle collisions and reverse horizontal velocity of the ball (and increase it slightly!)
                if self.ball_hit_paddle() {
                    log::warn!("Paddle hit!");
                    self.ball.vel.x *= -1.1;
                }

                // Check for ball-on-wall collisions act accordingly
                match self.ball_hit_wall() {
                    // If it hit the top or bottom wall, just reverse the vertical velocity of the ball (and increase it slightly!)
                    Some(Wall::Top) | Some(Wall::Bottom) => {
                        log::warn!("Top wall hit!");
                        self.ball.vel.y *= -1.1;
                    }

                    // If it hit the left wall, either score a point for P2 in a 2 player game, or dock a point from P1 in a 1 player game
                    Some(Wall::Left) => {
                        log::warn!("Left wall hit!");

                        match self.mode {
                            Mode::OnePlayer(_) => {
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
                self.ball = Ball::random();
            }
            _ => self.pause_for -= 1,
        }

        Ok(())
    }

    /// Draw the game screen
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        log::info!("Drawing!");

        // Clear the screen to white
        graphics::clear(ctx, Color::from_rgba(0, 0, 0, 255));

        // Create the ball mesh
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball.rect,
            Color::from_rgba(255, 255, 255, 255),
        )
        .expect("Error creating ball_mesh!");

        // Create the left paddle mesh
        let paddle_left_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.paddle_left,
            Color::from_rgba(255, 255, 255, 255),
        )
        .expect("Error creating paddle_left_mesh!");

        // Create the right paddle mesh
        let paddle_right_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.paddle_right,
            Color::from_rgba(255, 255, 255, 255),
        )
        .expect("Error creating paddle_right_mesh!");

        // Draw the ball
        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())
            .expect("Error drawing ball_mesh!");

        // Draw the left paddle
        graphics::draw(ctx, &paddle_left_mesh, graphics::DrawParam::default())
            .expect("Error drawing paddle_left_mesh!");

        // Draw the right paddle
        graphics::draw(ctx, &paddle_right_mesh, graphics::DrawParam::default())
            .expect("Error drawing paddle_right_mesh!");

        // Create the scoreboard text
        let mut scoreboard_text = match self.mode {
            Mode::OnePlayer(_) => {
                graphics::Text::new(format!("{0: <10}{1:03}", "P1", self.score.p1))
            }
            Mode::TwoPlayer(_, _) => graphics::Text::new(format!(
                "{0: <10}{1:03} | {2:03}{3: >10}",
                "P1", self.score.p1, self.score.p2, "P2"
            )),
        };
        scoreboard_text.set_font(graphics::Font::default(), PxScale::from(36.0));

        // This is where we'll draw the scoreboard
        let coords = [
            SCREEN_WIDTH / 2.0 - scoreboard_text.width(ctx) as f32 / 2.0,
            20.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        graphics::draw(ctx, &scoreboard_text, params).expect("Error drawing scoreboard text!");

        // Show the FPS counter
        let fps = ggez::timer::fps(ctx) as i64;
        let debug_text = graphics::Text::new(format!(
            "[fps: {}] [vel: {:.3},{:.3} | spd: {:.3}] [t: {:.1}]",
            fps,
            self.ball.vel.x,
            self.ball.vel.y,
            (self.ball.vel.x.hypot(self.ball.vel.y)),
            ggez::timer::duration_to_f64(ggez::timer::time_since_start(ctx))
        ));
        let params = graphics::DrawParam::default()
            .dest([20.0, SCREEN_HEIGHT - 20.0 - debug_text.height(ctx) as f32]);
        graphics::draw(ctx, &debug_text, params).expect("Error drawing debug text!");

        // Update the screen
        graphics::present(ctx).expect("Error presenting graphics!");

        Ok(())
    }
}

#[derive(Debug)]
struct Ball {
    rect: Rect,
    vel: Vector2<f32>,
}

impl Ball {
    fn new(x: f32, y: f32, radius: f32) -> Ball {
        Ball {
            rect: Rect::new(x, y, radius, radius),
            vel: Vector2::<f32> { x: 1.0, y: -0.5 },
        }
    }

    fn random() -> Ball {
        use rand::prelude::*;

        let mut prng = rand::thread_rng();

        let mut random_velocity = || -> f32 {
            let flip = prng.gen::<bool>();
            let mut vel = prng.gen_range(BALL_MIN_VEL..=BALL_MAX_VEL);
            // TODO: Account for edge case where both velocities are randomly set to 0.0
            if flip {
                vel = -vel;
            }
            vel
        };

        let vel_x = random_velocity();
        let vel_y = random_velocity();

        Ball {
            rect: Rect::new(
                SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                BALL_RADIUS,
                BALL_RADIUS,
            ),
            vel: Vector2::<f32> { x: vel_x, y: vel_y },
        }
    }
}

#[derive(Debug, Default)]
struct Score {
    pub p1: i16,
    pub p2: i16,
}
