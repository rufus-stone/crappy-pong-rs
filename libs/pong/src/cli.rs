use structopt::StructOpt;

use crate::player::*;
use crate::settings::*;

#[derive(StructOpt, Debug)]
pub struct Opt {
    // Game mode
    /// 1 = Human vs Human, 2 = Human vs AI, 3 = AI vs Human, 4 = AI vs AI, 5 = Human only, 6 = AI only
    #[structopt(short, long, default_value = "1")]
    pub mode: u8,

    // Target FPS
    /// Target frames per second (0 = unlimited)
    #[structopt(short, long, default_value = "0")]
    pub fps: u8,
}

#[derive(Debug, Clone)]
pub struct ModeError;

impl std::fmt::Display for ModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Invalid game mode selected. Defaulting to 2 player (Human vs Human)..."
        )
    }
}

pub fn get_game_mode() -> Result<Mode, ModeError> {
    // Read command line args, if any
    let args = Opt::from_args();

    match &args.mode {
        1 => Ok(PLAYER_VS_PLAYER),
        2 => Ok(PLAYER_VS_AI),
        3 => Ok(AI_VS_PLAYER),
        4 => Ok(AI_VS_AI),
        5 => Ok(PLAYER_VS_SELF),
        6 => Ok(AI_VS_SELF),
        _ => Err(ModeError),
    }
}
