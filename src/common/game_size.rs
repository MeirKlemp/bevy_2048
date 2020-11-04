use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct GameSize(f32);

impl GameSize {
    pub fn board_size(&self) -> f32 {
        self.0
    }

    pub fn tile_size(&self) -> f32 {
        (self.0 * 0.85) / 4.0
    }

    pub fn tile_spacing(&self) -> f32 {
        (self.0 * 0.15) / 5.0
    }

    pub fn merge_size(&self) -> f32 {
        self.tile_size() * 0.1
    }

    fn calculate_game_size(&mut self, width: f32, height: f32) {
        let (width, height) = (width * 0.9, height * 0.9);
        self.0 = height.min(width * 0.6);
    }
}

impl Default for GameSize {
    fn default() -> Self {
        Self(500.0)
    }
}

pub fn update_game_size(mut game_size: ResMut<GameSize>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    game_size.calculate_game_size(window.width() as f32, window.height() as f32);
}
