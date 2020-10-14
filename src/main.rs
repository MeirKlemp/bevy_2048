mod components;
mod movement;
mod score;
mod tile_spawning;
mod ui;

use bevy::{prelude::*, render::pass::ClearColor};
use components::{GameState, Position, Tile};
use movement::MovementPlugin;
use score::{Score, ScoreSystemPlugin};
use tile_spawning::{Despawn, SpawnTileEvent, SpawnTilePlugin};
use ui::UiPlugin;

#[macro_use]
extern crate savefile_derive;

/// The size of the whole board.
pub const BOARD_SIZE: f32 = 500.0;
/// The size of each tile.
pub const TILE_SIZE: f32 = (BOARD_SIZE * 0.85) / 4.0;
/// The space between two tiles.
pub const TILE_SPACING: f32 = (BOARD_SIZE * 0.15) / 5.0;

pub const MERGE_SIZE: f32 = 20.0;

/// Number of tiles to spawn at start.
pub const STARTING_TILES: usize = 2;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(SpawnTilePlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(ScoreSystemPlugin)
        .add_plugin(UiPlugin)
        .init_resource::<GameState>()
        // Set background color.
        .add_resource(ClearColor(Color::rgb_u8(250, 248, 239)))
        .add_startup_system(setup.system())
        .add_system(new_game.system())
        .add_system(space_new_game.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_tile_events: ResMut<Events<SpawnTileEvent>>,
) {
    commands
        // Cameras.
        .spawn(Camera2dComponents::default())
        // Board background.
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb_u8(119, 110, 101).into()),
            sprite: Sprite::new(Vec2::new(BOARD_SIZE, BOARD_SIZE)),
            ..Default::default()
        });

    // Creating a grid of empty tiles.
    for row in 0..4 {
        for col in 0..4 {
            let position = Position { row, col };

            commands.spawn(SpriteComponents {
                material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
                sprite: Sprite::new(Vec2::new(TILE_SIZE, TILE_SIZE)),
                transform: Transform::from_translation(position.into()),
                ..Default::default()
            });
        }
    }

    // Spawning tiles at the beginning.
    spawn_tile_events.send(SpawnTileEvent {
        count: STARTING_TILES,
    });
}

fn new_game(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut spawn_tile_events: ResMut<Events<SpawnTileEvent>>,
    mut score: ResMut<Score>,
    mut tiles: Query<With<Tile, Entity>>,
) {
    if *game_state == GameState::Restarting {
        for entity in &mut tiles.iter() {
            commands.insert_one(entity, Despawn);
        }

        spawn_tile_events.send(SpawnTileEvent {
            count: STARTING_TILES,
        });

        score.0 = 0;
        *game_state = GameState::Play;
    }
}

fn space_new_game(mut game_state: ResMut<GameState>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        *game_state = GameState::Restarting;
    }
}
