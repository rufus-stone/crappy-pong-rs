use rand::RngCore;

use ggez::graphics::Rect;

use ga::statistics::Statistics;
use genetic_algorithm as ga;

use crate::entities::paddle::Paddle;
use crate::game::engine::move_paddle;
use crate::players::ai::player::AiPlayer;
use crate::players::Move;
use crate::players::Snapshot;
use crate::settings::Config;
use crate::settings::*;

use super::score::*;

pub struct SimGame {
    paddle_left: Rect,
    paddle_right: Rect,
    ball: Ball,
    score: Score,
    serves: usize,
    ai_player: AiPlayer,
    finished: bool,
}

impl SimGame {
    /// Create a new SimGame with the specified AI player
    pub fn new(config: &Config, ai_player: AiPlayer) -> SimGame {
        SimGame {
            paddle_left: Rect::new(
                X_OFFSET,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),

            paddle_right: Rect::new(
                SCREEN_WIDTH - X_OFFSET - PADDLE_WIDTH,
                0.0,
                PADDLE_WIDTH,
                SCREEN_HEIGHT,
            ),
            ball: Ball::random(),
            score: Score::default(),
            serves: config.generation_length,
            ai_player,
            finished: false,
        }
    }

    /// Create a new SimGame with a randomly generated AI player
    pub fn random(config: &Config, prng: &mut dyn RngCore) -> SimGame {
        SimGame {
            paddle_left: Rect::new(
                X_OFFSET,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),

            paddle_right: Rect::new(
                SCREEN_WIDTH - X_OFFSET - PADDLE_WIDTH,
                0.0,
                PADDLE_WIDTH,
                SCREEN_HEIGHT,
            ),
            ball: Ball::random(),
            score: Score::default(),
            serves: config.generation_length,
            ai_player: AiPlayer::random(&config, prng),
            finished: false,
        }
    }

    /// Check if the ball hit a paddle
    pub fn ball_hit_paddle(&mut self) -> Option<Paddle> {
        if self.ball.vel.x < 0.0 && self.ball.rect.overlaps(&self.paddle_left) {
            self.score.p1 += 1;
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

    /// Checks for AI player input and moves the paddles accordingly
    pub fn handle_input(&mut self, ctx: &mut Context) {
        // Make sure this isn't the end of the simulation
        if self.serves > 1 {
            // Check AI player input
            let p1_move = self
                .ai_player
                .make_move(ctx, &Snapshot::new(&self.paddle_left, &self.ball));

            // Move paddle accordingly
            move_paddle(&mut self.paddle_left, p1_move);

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

                // If it hit the left wall, dock a point from the AI player, reset the ball and paddle, and chalk up one play through
                Some(Wall::Left) => {
                    log::warn!("Left wall hit!");
                    self.score.p1 -= 1;

                    // Reset the ball
                    self.ball = Ball::random();

                    // Reset the paddles
                    self.reset_paddle();

                    // Chalk off a game
                    self.serves -= 1;
                }

                // In an AI training game the right wall should never get hit
                Some(Wall::Right) => {
                    log::warn!("Right wall hit... Erm, this should never happen!");
                }

                None => {}
            }
        } else {
            self.finished = true;
        }
    }

    pub fn reset_paddle(&mut self) {
        self.paddle_left = Rect::new(
            X_OFFSET,
            SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );
    }
}

pub struct Simulation {
    games: Vec<SimGame>,
    config: Config,
}

impl Simulation {
    /// Create a new Simulation with randomly generated AiPlayers
    pub fn new(config: &Config, prng: &mut dyn RngCore) -> Simulation {
        log::warn!(
            "Creating new Simulation with {} trainees",
            config.population_size
        );

        let games = (0..config.population_size)
            .map(|_| SimGame::random(&config, prng))
            .collect();

        Simulation {
            games,
            config: config.clone(),
        }
    }

    /// Get immutable ref to the current set of training games
    pub fn games(&self) -> &[SimGame] {
        &self.games
    }

    /// Progress all the simulated games by one frame
    pub fn step(&mut self, ctx: &mut Context) -> Option<Statistics> {
        // For each unfinished simulated game, advance one frame
        self.games
            .iter_mut()
            .filter(|sim| !sim.finished)
            .for_each(|sim| {
                sim.handle_input(ctx);
            });

        // If all sims are done, try evolving
        if self.all_finished() {
            log::warn!("All sims finished!");
            //self.try_evolving(prng)
        }

        None
    }

    fn all_finished(&self) -> bool {
        self.games.as_slice().iter().all(|sim| sim.finished)
    }
}

use ggez::event;
use ggez::{Context, GameResult};
use glam::*;

use crate::entities::{ball::*, wall::*};

impl event::EventHandler for Simulation {
    /// Called every frame
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        log::warn!("Updating!!!");
        self.step(ctx);
        Ok(())
    }

    /// Draw the game screen
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_simulation() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // Create a new default config
        let config = Config::default();

        // Create a new random Simulation
        let simulation = Simulation::new(&config, &mut prng);

        // Get the weights for the network of the AiPlayer in the first SimGame
        let weights1: Vec<f32> = simulation.games()[0]
            .ai_player
            .brain
            .network()
            .weights()
            .collect();

        // Get the weights for the network of the second AiPlayer
        let weights2: Vec<f32> = simulation.games()[1]
            .ai_player
            .brain
            .network()
            .weights()
            .collect();

        // Check that the AiPlayers are actually different
        approx::assert_relative_ne!(weights1.as_slice(), weights2.as_slice());
    }
}
