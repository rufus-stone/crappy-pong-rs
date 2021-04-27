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

pub trait Move {
    fn make_move(&self, ctx: &mut ggez::Context) -> f32;
    fn name(&self) -> &'static str;
}

impl std::fmt::Debug for dyn Move {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Move {{ member: {:?} }}", self.name())
    }
}
#[derive(Debug)]
pub struct Controls {
    up: keyboard::KeyCode,
    down: keyboard::KeyCode,
}

#[derive(Debug)]
pub struct HumanPlayer {
    controls: Controls,
}

impl HumanPlayer {
    pub fn new(up: keyboard::KeyCode, down: keyboard::KeyCode) -> HumanPlayer {
        let player = HumanPlayer {
            controls: Controls { up, down },
        };

        log::warn!("New human player: {:?}", &player);

        player
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

    fn name(&self) -> &'static str {
        "Human"
    }
}
