use ggez::event;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard;
use ggez::mint::*;
use ggez::{Context, GameResult};
use glam::*;

//type Vector2 = ggez::mint::Vector2<f32>;

use crate::settings::*;

#[derive(Debug)]
pub struct GameState {
    paddle_left: Rect,
    paddle_right: Rect,
    ball: Ball,
    score: Score,
    pause_for: u64,
}

impl GameState {
    pub fn new() -> GameResult<GameState> {
        Ok(GameState {
            paddle_left: Rect::new(
                X_OFFSET,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            paddle_right: Rect::new(
                SCREEN_WIDTH - X_OFFSET - PADDLE_WIDTH,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            ball: Ball::new(
                SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                BALL_RADIUS,
            ),
            score: Score::default(),
            pause_for: 0,
        })
    }
}

/// Moves the paddles but prevents them from moving off the screen
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
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Only handle key presses if the game isn't paused
        match self.pause_for {
            0 => {
                // Check for key presses and move the paddles accordingly
                if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
                    move_paddle(&mut self.paddle_left, -PADDLE_SPEED);
                }

                if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S) {
                    move_paddle(&mut self.paddle_left, PADDLE_SPEED);
                }

                if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Up) {
                    move_paddle(&mut self.paddle_right, -PADDLE_SPEED);
                }

                if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Down) {
                    move_paddle(&mut self.paddle_right, PADDLE_SPEED);
                }

                // Move the ball based on its velocity
                self.ball.rect.translate(self.ball.vel);

                // Check for ball-on-paddle collisions and reverse horizontal velocity of the ball (and increase it slightly!)
                if (self.ball.vel.x < 0.0 && self.ball.rect.overlaps(&self.paddle_left))
                    || (self.ball.vel.x > 0.0 && self.ball.rect.overlaps(&self.paddle_right))
                {
                    self.ball.vel.x *= -1.1;
                }

                // Check for ball-on-wall collisions and reverse vertical velocity of the ball (and increase it slightly!)
                if (self.ball.vel.y < 0.0 && self.ball.rect.top() < 0.0)
                    || (self.ball.vel.y > 0.0
                        && self.ball.rect.bottom() > SCREEN_HEIGHT - BALL_RADIUS)
                {
                    self.ball.vel.y *= -1.1;
                }

                // If the ball hits the left wall, score a point for Player 2
                if self.ball.rect.left() < 0.0 {
                    self.score.p2 += 1;

                    // Wait for a second
                    //std::thread::sleep(std::time::Duration::from_secs(1));

                    // Pause for 1 second's worth of frames
                    self.pause_for = ggez::timer::fps(ctx) as u64;

                    // Reset the ball
                    //self.ball = Ball::random();

                    // ...or if the ball hits the right wall, score a point for Player 1
                } else if self.ball.rect.right() > SCREEN_WIDTH - BALL_RADIUS {
                    self.score.p1 += 1;

                    // Wait for a second
                    //std::thread::sleep(std::time::Duration::from_secs(1));
                    // Pause for 1 second's worth of frames
                    self.pause_for = ggez::timer::fps(ctx) as u64;

                    // Reset the ball
                    //self.ball = Ball::random();
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
        let mut scoreboard_text = graphics::Text::new(format!(
            "{0: <10}{1:03} | {2:03}{3: >10}",
            "P1", self.score.p1, self.score.p2, "P2"
        ));
        scoreboard_text.set_font(
            graphics::Font::default(),
            PxScale::from(36.0), //ggez::graphics::Scale::uniform(36.0),
        );

        // This is where we'll draw the scoreboard
        let coords = [
            SCREEN_WIDTH / 2.0 - scoreboard_text.width(ctx) as f32 / 2.0,
            20.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        graphics::draw(ctx, &scoreboard_text, params).expect("Error drawing scoreboard text!");

        // Show the FPS counter
        let fps = ggez::timer::fps(ctx) as i64;
        let fps_text = graphics::Text::new(format!(
            "[fps: {}] [vel: {:.3},{:.3}]",
            fps, self.ball.vel.x, self.ball.vel.y
        ));
        let params = graphics::DrawParam::default()
            .dest([20.0, SCREEN_HEIGHT - 20.0 - fps_text.height(ctx) as f32]);
        graphics::draw(ctx, &fps_text, params).expect("Error drawing FPS text!");

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
    pub p1: u16,
    pub p2: u16,
}
