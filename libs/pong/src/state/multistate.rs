use super::gamestate::GameState;

#[derive(Debug)]
pub struct MultiGameState {
    pub(crate) game_states: Vec<GameState>,
}
