use bevy::prelude::*;

use crate::components::{GameState, Position, Tile};

use super::MovingState;

pub fn check_moveable(
    mut game_state: ResMut<GameState>,
    mut moving_state: ResMut<MovingState>,
    mut tiles: Query<(&Tile, &Position)>,
) {
    if *game_state == GameState::Play {
        if *moving_state == MovingState::CheckingMoveable {
            let len = tiles.iter().iter().len();
            if len == 16 {
                let mut iter = tiles.iter();
                let null_tile = Tile { level: 0 };
                let mut board = [&null_tile; 16];
                for (tile, position) in &mut iter {
                    board[position.index()] = tile;
                }

                let mut gameover = true;
                for row in 0..4 {
                    for col in 0..4 {
                        let pos = Position { row, col };

                        if row < 3 {
                            let up = Position { row: row + 1, col };
                            if board[pos.index()].level == board[up.index()].level {
                                gameover = false;
                                break;
                            }
                        }

                        if col < 3 {
                            let right = Position { row, col: col + 1 };
                            if board[pos.index()].level == board[right.index()].level {
                                gameover = false;
                                break;
                            }
                        }
                    }
                }

                if gameover {
                    *game_state = GameState::GameOver;
                }
            }

            *moving_state = MovingState::Idle;
        }
    }
}