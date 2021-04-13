use ggez::input::keyboard;

use crate::settings::*;

#[derive(Debug)]
pub enum Mode {
    OnePlayer(Player),
    TwoPlayer(Player, Player),
}

#[derive(Debug)]
pub enum Player {
    Human,
    Computer,
}

trait Move {
    fn make_move(&self, ctx: &mut ggez::Context) -> f32;
}

pub struct Controls {
    up: keyboard::KeyCode,
    down: keyboard::KeyCode,
}

pub struct HumanPlayer {
    controls: Controls,
}

impl HumanPlayer {
    pub fn new(up: keyboard::KeyCode, down: keyboard::KeyCode) -> HumanPlayer {
        HumanPlayer {
            controls: Controls { up, down },
        }
    }
}

impl Move for HumanPlayer {
    fn make_move(&self, ctx: &mut ggez::Context) -> f32 {
        // Check for key presses and move Player 1 paddle accordingly
        if keyboard::is_key_pressed(ctx, self.controls.up) {
            -PADDLE_SPEED
        } else if keyboard::is_key_pressed(ctx, self.controls.down) {
            PADDLE_SPEED
        } else {
            0.0
        }
    }
}

pub struct AiPlayer {}

impl Move for AiPlayer {
    fn make_move(&self, ctx: &mut ggez::Context) -> f32 {
        todo!()
    }
}
