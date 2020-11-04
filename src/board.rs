use bevy::prelude::*;

use crate::{
    common::{GameSize, Position, Tile},
    movement::Moving,
    tile_spawning::SpawnAnimation,
};

pub struct Board;

pub struct EmptyTile;

pub fn spawn_board(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_size: Res<GameSize>,
) {
    // Board background.
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb_u8(119, 110, 101).into()),
            sprite: Sprite::new(Vec2::new(game_size.board_size(), game_size.board_size())),
            ..Default::default()
        })
        .with(Board);

    // Creating a grid of empty tiles.
    for row in 0..4 {
        for col in 0..4 {
            let position = Position { row, col };

            commands
                .spawn(SpriteComponents {
                    material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
                    sprite: Sprite::new(Vec2::new(game_size.tile_size(), game_size.tile_size())),
                    transform: Transform::from_translation(position.to_vec3(*game_size)),
                    ..Default::default()
                })
                .with(position)
                .with(EmptyTile);
        }
    }
}

pub fn update_board_size(game_size: Res<GameSize>, mut sprite: Mut<Sprite>, _: &Board) {
    sprite.size = Vec2::new(game_size.board_size(), game_size.board_size());
}

pub fn update_tiles_size_and_position(
    game_size: Res<GameSize>,
    mut tiles_size: Query<With<Tile, Without<SpawnAnimation, &mut Sprite>>>,
    mut tiles_position: Query<(&mut Transform, &Position, &Option<Moving>)>,
    mut empty_tiles_size: Query<With<EmptyTile, &mut Sprite>>,
    mut empty_tiles_position: Query<With<EmptyTile, (&mut Transform, &Position)>>,
) {
    for mut sprite in tiles_size.iter_mut() {
        sprite.size = Vec2::new(game_size.tile_size(), game_size.tile_size());
    }

    for (mut transform, position, moving) in tiles_position.iter_mut() {
        if moving.is_none() {
            transform.translation = position.to_vec3(*game_size);
        }
    }

    for mut sprite in empty_tiles_size.iter_mut() {
        sprite.size = Vec2::new(game_size.tile_size(), game_size.tile_size());
    }

    for (mut transform, position) in empty_tiles_position.iter_mut() {
        transform.translation = position.to_vec3(*game_size);
    }
}
