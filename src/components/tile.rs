use bevy::prelude::*;

/// Component for saving tile level.
#[derive(Debug)]
pub struct Tile {
    pub level: u32,
}

impl Tile {
    /// Each level has a unique color (up to 9).
    /// Returns the color for a given tile.
    pub fn color(&self) -> Color {
        match self.level {
            0 => Color::rgb_u8(255, 255, 0),  // Yellow
            1 => Color::rgb_u8(255, 69, 0),   // Orange Red
            2 => Color::rgb_u8(255, 0, 0),    // Red
            3 => Color::rgb_u8(255, 0, 255),  // Magenta
            4 => Color::rgb_u8(75, 0, 130),   // Indigo
            5 => Color::rgb_u8(0, 0, 255),    // Blue
            6 => Color::rgb_u8(0, 255, 255),  // Cyan
            7 => Color::rgb_u8(0, 255, 0),    // Green
            8 => Color::rgb_u8(139, 69, 19),  // Saddle Brown
            9 => Color::rgb_u8(184, 134, 11), // Dark Golden Rod
            _ => Color::BLACK,
        }
    }

    /// Calculates the score of a given tile (pow(2, level)).
    pub fn score(&self) -> u32 {
        2u32.pow(self.level + 1)
    }
}
/// Component for saving the position of a tile in the grid.
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    /// Calculates the index of the position on a board
    /// represented by a 1D array.
    pub fn index(&self) -> usize {
        self.row * 4 + self.col
    }
}

impl From<Position> for Vec3 {
    /// Transforms a position into a world point.
    fn from(pos: Position) -> Self {
        // Offset from the bottom left point of the board.
        let offset = Vec3::new(
            -(crate::BOARD_SIZE - crate::TILE_SIZE) / 2.0 + crate::TILE_SPACING,
            -(crate::BOARD_SIZE - crate::TILE_SIZE) / 2.0 + crate::TILE_SPACING,
            0.0,
        );

        Vec3::new(
            (crate::TILE_SIZE + crate::TILE_SPACING) * pos.col as f32,
            (crate::TILE_SIZE + crate::TILE_SPACING) * pos.row as f32,
            0.0,
        ) + offset
    }
}
