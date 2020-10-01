mod animation;

use animation::Animation;
use bevy::{prelude::*, render::pass::ClearColor};
use rand::Rng;
use std::convert::TryFrom;

/// The size of the whole board.
const BOARD_SIZE: f32 = 500.0;
/// The size of each tile.
const TILE_SIZE: f32 = (BOARD_SIZE * 0.85) / 4.0;
/// The space between two tiles.
const TILE_SPACING: f32 = (BOARD_SIZE * 0.15) / 5.0;

/// Number of tiles to spawn at start.
const STARTING_TILES: u32 = 2;
/// The maximum level(exluded) of a spawned tile.
const MAX_STARTING_LEVEL: u32 = 2;

fn main() {
    App::build()
        .add_default_plugins()
        .add_event::<SpawnTileEvent>()
        .init_resource::<SpawnTileListener>()
        .init_resource::<MovingState>()
        .init_resource::<MovingAnimation>()
        .add_resource(MovingDirection::Left)
        // Set background color.
        .add_resource(ClearColor(Color::rgb_u8(250, 248, 239)))
        .add_startup_system(setup.system())
        .add_system(spawn_tiles.system())
        .add_system(spawn_animation.system())
        .add_system(moving_input.system())
        .add_system(set_moving.system())
        .add_system(moving_animation.system())
        .add_system(merging.system())
        .add_system(finish_moving.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_tile_events: ResMut<Events<SpawnTileEvent>>,
) {
    commands
        // Camera.
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
    for _ in 0..STARTING_TILES {
        spawn_tile_events.send(SpawnTileEvent);
    }
}

/// Component for saving tile level.
#[derive(Debug)]
struct Tile {
    level: u32,
}

impl Tile {
    /// Each level has a unique color (up to 9).
    /// Returns the color for a given tile.
    fn color(&self) -> Color {
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
    fn score(&self) -> u32 {
        2u32.pow(self.level + 1)
    }
}

/// Event for spawning new tiles.
struct SpawnTileEvent;

/// Event listener for SpawnTileEvent.
#[derive(Default)]
struct SpawnTileListener {
    reader: EventReader<SpawnTileEvent>,
}

/// Component for saving the position of a tile in the grid.
#[derive(Debug, Default, PartialEq, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    /// Calculates the index of the position on a board
    /// represented by a 1D array.
    fn index(&self) -> usize {
        self.row * 4 + self.col
    }
}

impl From<Position> for Vec3 {
    /// Transforms a position into a world point.
    fn from(pos: Position) -> Self {
        // Offset from the bottom left point of the board.
        let offset = Vec3::new(
            -(BOARD_SIZE - TILE_SIZE) / 2.0 + TILE_SPACING,
            -(BOARD_SIZE - TILE_SIZE) / 2.0 + TILE_SPACING,
            0.0,
        );

        Vec3::new(
            (TILE_SIZE + TILE_SPACING) * pos.col as f32,
            (TILE_SIZE + TILE_SPACING) * pos.row as f32,
            0.0,
        ) + offset
    }
}

/// Component used to animate the tiles spawning.
struct SpawnAnimation {
    animation: Animation,
}

impl Default for SpawnAnimation {
    /// Sets the animation to finish after 3 updates.
    fn default() -> Self {
        Self {
            animation: Animation::new(3),
        }
    }
}

/// Animating each tile that contains SpawnAnimation component.
/// When the animation is finished, the SpawnAnimation component
/// is removed from the entity.
fn spawn_animation(
    mut commands: Commands,
    time: Res<Time>,
    entity: Entity,
    mut spawn_anim: Mut<SpawnAnimation>,
    mut sprite: Mut<Sprite>,
) {
    if spawn_anim.animation.update(time.delta_seconds) {
        // Updating the sprite size while the animation is not finished.
        let size = TILE_SIZE * spawn_anim.animation.value();
        sprite.size.set_x(size);
        sprite.size.set_y(size);
    }

    // When the animation is finished, the component is being removed.
    if spawn_anim.animation.finished() {
        commands.remove_one::<SpawnAnimation>(entity);
    }
}

/// Spawning a new tile for every SpawnTileEvent event.
fn spawn_tiles(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut listener: ResMut<SpawnTileListener>,
    spawn_events: Res<Events<SpawnTileEvent>>,
    mut positions: Query<&Position>,
) {
    // Vector of empty tiles for all the iterations.
    let mut free_pos = None;
    for _ in listener.reader.iter(&spawn_events) {
        if free_pos.is_none() {
            // Creating vector of empty tiles.
            let mut vec = Vec::new();
            for row in 0..4 {
                for col in 0..4 {
                    vec.push(Position { row, col });
                }
            }

            // Removing the existing tiles from the vector.
            for pos in &mut positions.iter() {
                if let Some(idx) = vec.iter().position(|x| *x == *pos) {
                    vec.remove(idx);
                }
            }

            free_pos = Some(vec);
        }

        let vec = free_pos.as_mut().unwrap();

        // Checking that the board is not full.
        if vec.len() != 0 {
            // Choosing a random empty tile.
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0, vec.len());
            let pos = vec.remove(idx);

            // Choosing the new tile's level.
            let tile = Tile {
                level: rng.gen_range(0, MAX_STARTING_LEVEL),
            };

            // Spawning the new tile.
            commands
                .spawn(SpriteComponents {
                    material: materials.add(tile.color().into()),
                    transform: Transform::from_translation(pos.into()),
                    ..Default::default()
                })
                .with(tile)
                .with(pos)
                .with(SpawnAnimation::default())
                .with(Option::<Moving>::None)
                .with(Option::<Merged>::None);
        } else {
            #[cfg(debug_assertions)]
            panic!("spawn_tiles(): Tried to spawn a tile when the board was full.")
        }
    }
}

#[derive(Debug, PartialEq)]
enum MovingState {
    Idle,
    SetMoving { starting: bool },
    Animating,
    Merging,
    Finishing { moved: bool },
}

impl Default for MovingState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum MovingDirection {
    Left,
    Up,
    Right,
    Down,
}

impl MovingDirection {
    fn moved_position(&self, position: &Position) -> Option<Position> {
        match self {
            Self::Left if position.col > 0 => Some(Position {
                row: position.row,
                col: position.col - 1,
            }),
            Self::Up if position.row < 3 => Some(Position {
                row: position.row + 1,
                col: position.col,
            }),
            Self::Right if position.col < 3 => Some(Position {
                row: position.row,
                col: position.col + 1,
            }),
            Self::Down if position.row > 0 => Some(Position {
                row: position.row - 1,
                col: position.col,
            }),
            _ => None,
        }
    }

    fn board_iteration(&self) -> [Position; 16] {
        let mut result: [Position; 16] = Default::default();
        // let mut iter = result.iter_mut();
        let mut index = 0;

        for secondary in 0..4 {
            for mut primary in 0..4 {
                if let Self::Up | Self::Right = self {
                    primary = 3 - primary;
                }

                result[index] = match self {
                    Self::Left | Self::Right => Position {
                        row: secondary,
                        col: primary,
                    },
                    Self::Up | Self::Down => Position {
                        row: primary,
                        col: secondary,
                    },
                };

                index += 1;
            }
        }

        result
    }
}

impl TryFrom<&KeyCode> for MovingDirection {
    type Error = &'static str;
    fn try_from(key: &KeyCode) -> Result<Self, Self::Error> {
        match key {
            KeyCode::Left | KeyCode::A => Ok(Self::Left),
            KeyCode::Up | KeyCode::W => Ok(Self::Up),
            KeyCode::Right | KeyCode::D => Ok(Self::Right),
            KeyCode::Down | KeyCode::S => Ok(Self::Down),
            _ => Err("Couldn't convert the key into a direction"),
        }
    }
}

impl From<MovingDirection> for Vec3 {
    fn from(direction: MovingDirection) -> Self {
        match direction {
            MovingDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            MovingDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            MovingDirection::Right => Vec3::new(1.0, 0.0, 0.0),
            MovingDirection::Down => Vec3::new(0.0, -1.0, 0.0),
        }
    }
}

struct MovingAnimation {
    animation: Animation,
}

impl Default for MovingAnimation {
    fn default() -> Self {
        Self {
            animation: Animation::new(5),
        }
    }
}

struct Moving;

struct Merged;

fn moving_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut moving_state: ResMut<MovingState>,
    mut moving_dir: ResMut<MovingDirection>,
) {
    if *moving_state == MovingState::Idle {
        for key in keyboard_input.get_just_pressed() {
            if let Ok(direction) = MovingDirection::try_from(key) {
                *moving_state = MovingState::SetMoving { starting: true };
                *moving_dir = direction;
            }
        }
    }
}

fn set_moving(
    mut commands: Commands,
    mut moving_state: ResMut<MovingState>,
    moving_dir: Res<MovingDirection>,
    mut tiles: Query<(Entity, &Tile, &Position, &Option<Moving>, &Option<Merged>)>,
) {
    if let MovingState::SetMoving { starting } = *moving_state {
        let mut tiles = tiles.iter();
        let mut board: [Option<(Entity, &Tile, &Position, &Option<Moving>, &Option<Merged>)>; 16] =
            Default::default();
        for tile in &mut tiles {
            let position = tile.2;
            board[position.index()] = Some(tile);
        }

        let mut moving_entities = Vec::new();
        for curr_pos in moving_dir.board_iteration().iter() {
            if let Some(curr_tile) = &board[curr_pos.index()] {
                if let Some(new_pos) = moving_dir.moved_position(curr_pos) {
                    if let Some(existing_tile) = &board[new_pos.index()] {
                        if moving_entities.contains(&existing_tile.0)
                            || (curr_tile.1.level == existing_tile.1.level
                                && curr_tile.4.is_none()
                                && existing_tile.4.is_none())
                        {
                            moving_entities.push(curr_tile.0);
                        }
                    } else {
                        // If the new position is emtpy,
                        // move the current tile.
                        moving_entities.push(curr_tile.0);
                    }
                }
            }
        }

        let moving = !moving_entities.is_empty();

        for entity in moving_entities {
            commands.insert_one(entity, Some(Moving));
        }

        *moving_state = if moving {
            MovingState::Animating
        } else {
            MovingState::Finishing { moved: !starting }
        };
    }
}

fn moving_animation(
    time: Res<Time>,
    mut moving_state: ResMut<MovingState>,
    mut moving_anim: ResMut<MovingAnimation>,
    moving_dir: Res<MovingDirection>,
    mut animate_transform: Query<(&Position, &mut Transform, &Option<Moving>)>,
    mut update_position: Query<(&mut Position, &mut Option<Moving>)>,
) {
    if *moving_state == MovingState::Animating {
        if moving_anim.animation.update(time.delta_seconds) {
            for (position, mut transform, moving) in &mut animate_transform.iter() {
                if moving.is_some() {
                    let translate: Vec3 = Vec3::from(*moving_dir)
                        * (TILE_SIZE + TILE_SPACING)
                        * moving_anim.animation.value();
                    transform.set_translation(Vec3::from(*position) + translate);
                }
            }
        }

        if moving_anim.animation.finished() {
            for (mut position, mut moving) in &mut update_position.iter() {
                if moving.is_some() {
                    *position = moving_dir.moved_position(&position).unwrap();
                    *moving = None;
                }
            }

            moving_anim.animation.reset();
            *moving_state = MovingState::Merging;
        }
    }
}

fn merging(
    mut commands: Commands,
    mut moving_state: ResMut<MovingState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tiles: Query<(
        Entity,
        &mut Tile,
        &Position,
        &mut Option<Merged>,
        &mut Handle<ColorMaterial>,
    )>,
) {
    if *moving_state == MovingState::Merging {
        let mut board = [None; 16];
        for (entity, mut tile, position, mut merged, mut material) in &mut tiles.iter() {
            if let Some((existing_entity, _position)) = board[position.index()] {
                tile.level += 1;
                *material = materials.add(tile.color().into());
                *merged = Some(Merged);

                commands.despawn(existing_entity);
            } else {
                board[position.index()] = Some((entity, position));
            }
        }

        *moving_state = MovingState::SetMoving { starting: false };
    }
}

fn finish_moving(
    mut moving_state: ResMut<MovingState>,
    mut spawn_tile_events: ResMut<Events<SpawnTileEvent>>,
    mut merged: Query<&mut Option<Merged>>,
) {
    if let MovingState::Finishing { moved } = *moving_state {
        for mut merged in &mut merged.iter() {
            if merged.is_some() {
                *merged = None;
            }
        }

        if moved {
            spawn_tile_events.send(SpawnTileEvent);
        }

        *moving_state = MovingState::Idle
    }
}
