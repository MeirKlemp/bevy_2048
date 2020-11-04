//! This module contains impl of common components.

mod animation;
pub use animation::Animation;

mod tile_components;
pub use tile_components::{Position, Tile};

mod game_state;
pub use game_state::GameState;

mod game_size;
pub use game_size::GameSize;
pub use game_size::update_game_size;