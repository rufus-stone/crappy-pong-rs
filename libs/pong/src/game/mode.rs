use crate::players::*;

#[derive(Debug)]
pub enum Mode {
    OnePlayer(Player),
    TwoPlayer(Player, Player),
    TrainAi(Player),
}
