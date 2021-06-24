use ggez::graphics;
use ggez::graphics::*;
use ggez::{Context, GameResult};
use glam::*;

use crate::player::*;
use crate::settings::*;

use crate::core::GameState;

impl GameState {
    /// Render the game screen
    pub fn render_game(&mut self, ctx: &mut Context) -> GameResult<()> {
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
            Mode::OnePlayer(_) | Mode::TrainAI(_) => {
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

    /// Render a splash while training
    pub fn render_training(&mut self, ctx: &mut Context) -> GameResult<()> {
        log::info!("Drawing!");

        // Clear the screen to white
        graphics::clear(ctx, Color::from_rgba(0, 0, 0, 255));

        // Create the scoreboard text
        let mut scoreboard_text =
            graphics::Text::new(format!("{0: <10}{1:03}", "P1", self.score.p1));
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
