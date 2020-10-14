//! This module contains the impl of the enum GameState.

// This enum tells in what state the game is in.
#[derive(Debug, PartialEq)]
pub enum GameState {
    Play,
    GameOver,
    Restarting,
}

impl Default for GameState {
    /// Creates a Play game state.
    fn default() -> Self {
        Self::Play
    }
}
