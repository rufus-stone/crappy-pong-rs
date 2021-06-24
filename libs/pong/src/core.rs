use ggez::event;
use ggez::{Context, GameResult};
use glam::*;

use crate::cli;
use crate::entities::{ball::*, wall::*};
use crate::game::mode::*;
use crate::state::gamestate::*;

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
                                        Mode::OnePlayer(_) | Mode::TrainAi(_) => {
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
            Mode::TrainAi(_) => {
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
            Mode::TrainAi(_) => self.render_training(ctx),
        }
    }
}
