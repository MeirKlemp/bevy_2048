#[derive(Debug, PartialEq)]
pub enum GameState {
    Play,
    GameOver,
}

impl Default for GameState {
    /// Creates a Play game state.
    fn default() -> Self {
        Self::Play
    }
}
