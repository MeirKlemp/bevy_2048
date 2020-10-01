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

const MERGE_SIZE: f32 = 20.0;

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
        .add_system(despawn_animation.system())
        .add_system(merge_animation.system())
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

/// Component used to animate the tiles despawning.
struct DespawnAnimation {
    animation: Animation,
}

impl Default for DespawnAnimation {
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

/// Despawning with an animation all tiles that have a despawn animation.
fn despawn_animation(
    mut commands: Commands,
    time: Res<Time>,
    entity: Entity,
    mut despawn_anim: Mut<DespawnAnimation>,
    mut sprite: Mut<Sprite>,
) {
    if despawn_anim.animation.update(time.delta_seconds) {
        // Updating the sprite size while the animation is not finished.
        let size = TILE_SIZE * despawn_anim.animation.rev_value();
        sprite.size.set_x(size);
        sprite.size.set_y(size);
    }

    // When the animation is finished, the entity will be despawned.
    if despawn_anim.animation.finished() {
        commands.despawn(entity);
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

/// The struct's aim is to cut the proccess of moving into
/// small pieces with states.
#[derive(Debug, PartialEq)]
enum MovingState {
    /// This is the default state, when no moving is happening.
    /// When should move the next state is `SetMoving` with
    /// `starting` set to `true`.
    Idle,
    /// At this state, checking which tile should move.
    /// When done checking, if some tiles should move,
    /// the next state is `Animating`
    /// otherwise, the next state is `Finishing` with
    /// `moved` set to `!starting`.
    SetMoving {
        /// Tells if this is the first time checking for moving tiles.
        starting: bool,
    },
    /// While at this state, all the tiles that should move are
    /// sliding in the moving direction.
    /// When done animating, the next state is `Merging`.
    Animating,
    /// At this state, checking each tiles that are at the same position,
    /// Are being merged.
    /// Then setting the next state to `SetMoving` with `starting` set to `false`.
    Merging,
    /// At this state, all the tiles are at their final position.
    /// Removing the merged compoent from the tiles and spawning a new
    /// tile if `moved` is `true`.
    /// When done, the next state is `Idle`.
    Finishing {
        /// Tells if any tile have been moved.
        moved: bool,
    },
}

impl Default for MovingState {
    /// Creates an Idle moving state.
    fn default() -> Self {
        Self::Idle
    }
}

/// The direction of the movement.
/// This is a global resource because all tiles
/// moving to the same direction.
#[derive(Debug, PartialEq, Copy, Clone)]
enum MovingDirection {
    Left,
    Up,
    Right,
    Down,
}

impl MovingDirection {
    /// Returns the new position after the movement according
    /// to the direction.
    /// Returns `None` if the new position is out of bounds.
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
            // If the new position is out of bounds.
            _ => None,
        }
    }

    /// Returns an array sorted by the order of tiles should
    /// be iterated when checking which tile should move.
    fn board_iteration(&self) -> [Position; 16] {
        let mut result: [Position; 16] = Default::default();
        let mut index = 0;

        // When moving to the left, secondary is the rows
        // because it doesn't matter which row should
        // be checked first.
        for secondary in 0..4 {
            // When moving to the left, primary is the columns
            // because the order of checking does matter.
            for mut primary in 0..4 {
                // Reversing primary.
                if let Self::Up | Self::Right = self {
                    primary = 3 - primary;
                }

                // Saving the position in the array.
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

    /// Converts the arrows and a,w,d,s keys into a direction.
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
    /// Converts a direction into a normalized vec3.
    fn from(direction: MovingDirection) -> Self {
        match direction {
            MovingDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            MovingDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            MovingDirection::Right => Vec3::new(1.0, 0.0, 0.0),
            MovingDirection::Down => Vec3::new(0.0, -1.0, 0.0),
        }
    }
}

/// Animating the movement of the tiles.
/// This is a global resource because all tiles
/// should be animated the same time.
struct MovingAnimation {
    animation: Animation,
}

impl Default for MovingAnimation {
    /// Sets the animation to finish after 3 updates.
    fn default() -> Self {
        Self {
            animation: Animation::new(3),
        }
    }
}

/// Component to tell if a tile is moving or not.
struct Moving;

/// Component to tell if a tile has been merged or not.
struct Merged;

/// While the moving state is `Idle`, getting the input
/// of the user.
/// If the user pressed the arrows or a,w,d,s keys,
/// the direction is being chosen
fn moving_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut moving_state: ResMut<MovingState>,
    mut moving_dir: ResMut<MovingDirection>,
) {
    if *moving_state == MovingState::Idle {
        // Iterating through the keys that were just pressed by the user.
        for key in keyboard_input.get_just_pressed() {
            // Checking if the keys can be converted into a direction
            if let Ok(direction) = MovingDirection::try_from(key) {
                // Setting the direction.
                *moving_dir = direction;
                // Setting the moving state to `SetMoving` with starting.
                *moving_state = MovingState::SetMoving { starting: true };
            }
        }
    }
}

// When the moving state is `SetMoving`, it checks which tile should move.
fn set_moving(
    mut commands: Commands,
    mut moving_state: ResMut<MovingState>,
    moving_dir: Res<MovingDirection>,
    mut tiles: Query<(Entity, &Tile, &Position, &Option<Moving>, &Option<Merged>)>,
) {
    // Checking the moving state.
    if let MovingState::SetMoving { starting } = *moving_state {
        // Creating a board represented by a 1D array
        // in order to check the neighbors tiles.
        let mut tiles = tiles.iter();
        let mut board: [Option<(Entity, &Tile, &Position, &Option<Moving>, &Option<Merged>)>; 16] =
            Default::default();
        for tile in &mut tiles {
            let position = tile.2;
            board[position.index()] = Some(tile);
        }

        // Vec of all the entities that should move.
        let mut moving_entities = Vec::new();

        // Iterate on the board according to the movement direction.
        for curr_pos in moving_dir.board_iteration().iter() {
            // Checking that a tile exists in the current position.
            if let Some(curr_tile) = &board[curr_pos.index()] {
                // Checking that the new position is not out of bounds.
                if let Some(new_pos) = moving_dir.moved_position(curr_pos) {
                    // Checking if the new position contains a tile.
                    if let Some(existing_tile) = &board[new_pos.index()] {
                        // If the existing tile is moving
                        // or has the same level while both tiles are not merged,
                        // move the current tile.
                        if moving_entities.contains(&existing_tile.0)
                            || (curr_tile.1.level == existing_tile.1.level
                                && curr_tile.4.is_none()
                                && existing_tile.4.is_none())
                        {
                            moving_entities.push(curr_tile.0);
                        }
                    } else {
                        // If the new position is emtpy, move the current tile.
                        moving_entities.push(curr_tile.0);
                    }
                }
            }
        }

        let moving = !moving_entities.is_empty();

        // Set the tiles that should move to `Moving`.
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

/// While the moving state is `Animating`, animating all moving tiles.
fn moving_animation(
    time: Res<Time>,
    mut moving_state: ResMut<MovingState>,
    mut moving_anim: ResMut<MovingAnimation>,
    moving_dir: Res<MovingDirection>,
    mut animate_transform: Query<(&Position, &mut Transform, &Option<Moving>)>,
    mut update_position: Query<(&mut Position, &mut Option<Moving>)>,
) {
    if *moving_state == MovingState::Animating {
        // Checking if should update the transform of the tiles.
        if moving_anim.animation.update(time.delta_seconds) {
            // For each tile that is moving, update its transform.
            for (position, mut transform, moving) in &mut animate_transform.iter() {
                if moving.is_some() {
                    // The amount to move from its position.
                    let translate: Vec3 = Vec3::from(*moving_dir)
                        * (TILE_SIZE + TILE_SPACING)
                        * moving_anim.animation.value();

                    // update the transform.
                    transform.set_translation(Vec3::from(*position) + translate);
                }
            }
        }

        // If the animation have been finished, remove all moving and
        // update the position component.
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

/// When the moving state is `Merging`, it merging tiles
/// that are in the same position.
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
        // Create a board with entity and position to check
        // if two tiles are at the same position.
        let mut board = [None; 16];
        for (entity, mut tile, position, mut merged, mut material) in &mut tiles.iter() {
            // Check if a tile is already exists at that position.
            if let Some((existing_entity, _position)) = board[position.index()] {
                // Despawning the existing tile.
                commands.despawn(existing_entity);

                // Checking that the level is not the last one.
                if tile.level < 9 {
                    // Updating current tile level and color.
                    tile.level += 1;
                    *material = materials.add(tile.color().into());

                    // Setting the tile as merged.
                    *merged = Some(Merged);
                    commands.insert_one(entity, MergeAnimation::default());
                } else {
                    // If the level is the last, despawn the tile with an animation.
                    commands.remove_one::<Tile>(entity);
                    commands.remove_one::<Position>(entity);
                    commands.remove_one::<Option<Moving>>(entity);
                    commands.remove_one::<Option<Merged>>(entity);
                    commands.insert_one(entity, DespawnAnimation::default());
                }
            }

            // Move the tile into the board.
            board[position.index()] = Some((entity, position));
        }

        *moving_state = MovingState::SetMoving { starting: false };
    }
}

/// When the moving state is `Finishing`, removing set all merged to `None`
/// and spawn a new tile.
fn finish_moving(
    mut moving_state: ResMut<MovingState>,
    mut spawn_tile_events: ResMut<Events<SpawnTileEvent>>,
    mut merged: Query<&mut Option<Merged>>,
) {
    if let MovingState::Finishing { moved } = *moving_state {
        // Setting all the merged to `None`.
        for mut merged in &mut merged.iter() {
            if merged.is_some() {
                *merged = None;
            }
        }

        // If some tiles have been moved, spawn a new tile.
        if moved {
            spawn_tile_events.send(SpawnTileEvent);
        }

        *moving_state = MovingState::Idle
    }
}

/// Component used to animate the tiles that have been merged.
struct MergeAnimation {
    animation: Animation,
}

impl Default for MergeAnimation {
    /// Sets the animation to finish after 8
    fn default() -> Self {
        let func = |x| 4.0 * x * (1.0 - x);

        Self {
            animation: Animation::with_func(8, func),
        }
    }
}

/// Animating all the tiles that have been merged.
fn merge_animation(
    mut commands: Commands,
    time: Res<Time>,
    entity: Entity,
    mut merge_anim: Mut<MergeAnimation>,
    mut sprite: Mut<Sprite>,
) {
    if merge_anim.animation.update(time.delta_seconds) {
        // Updating the sprite size while the animation is not finished.
        let size = TILE_SIZE + MERGE_SIZE * merge_anim.animation.value();
        sprite.size.set_x(size);
        sprite.size.set_y(size);
    }

    // When the animation is finished, the component is being removed.
    if merge_anim.animation.finished() {
        commands.remove_one::<MergeAnimation>(entity);
    }
}
