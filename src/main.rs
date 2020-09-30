use std::convert::TryFrom;

use bevy::{prelude::*, render::pass::ClearColor};
use rand::Rng;

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
        .add_resource(MovingDirection::Left)
        // Moving animation.
        .add_resource(Animation::new(5))
        // Set background color.
        .add_resource(ClearColor(Color::rgb_u8(250, 248, 239)))
        .add_startup_system(setup.system())
        .add_system(spawn_tiles.system())
        .add_system(spawn_animation.system())
        .add_system(moving_input.system())
        .add_system(set_moving.system())
        .add_system(moving_animation.system())
        .add_system(merging.system())
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

struct Animation {
    timer: Timer,
    ticks: usize,
    max_ticks: usize,
    finished: bool,
}

impl Animation {
    fn new(max_ticks: usize) -> Self {
        Self {
            max_ticks,
            ..Default::default()
        }
    }

    /// Returns a value in the range [0, 1] for the animation.
    fn value(&self) -> f32 {
        self.ticks as f32 / self.max_ticks as f32
    }

    /// Updates the animation, needs delta_seconds from the time resource.
    /// Returns `true` if the timer finished,
    /// which means the `value()` have been changed.
    fn update(&mut self, delta_seconds: f32) -> bool {
        if !self.finished {
            self.timer.tick(delta_seconds);

            if self.timer.finished {
                self.ticks += 1;
                if self.ticks >= self.max_ticks {
                    self.finished = true;
                }
            }

            return self.timer.finished;
        }

        false
    }

    /// Resets the animation.
    fn reset(&mut self) {
        self.timer.reset();
        self.ticks = 0;
        self.finished = false;
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / 60.0, true),
            ticks: 0,
            max_ticks: 10,
            finished: false,
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
    mut animation: Mut<Animation>,
    mut sprite: Mut<Sprite>,
) {
    if animation.update(time.delta_seconds) {
        // Updating the sprite size while the animation is not finished.
        let size = TILE_SIZE * animation.value();
        sprite.size.set_x(size);
        sprite.size.set_y(size);
    }

    // When the animation is finished, the component is being removed.
    if animation.finished {
        commands.remove_one::<Animation>(entity);
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
                .with(Animation::new(3))
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
    SetMoving,
    Animating,
    Merging,
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
            match self {
                Self::Left => {
                    for primary in 0..4 {
                        result[index] = Position {
                            row: secondary,
                            col: primary,
                        };
                        index += 1;
                    }
                }
                Self::Up => {
                    for primary in (0..4).rev() {
                        result[index] = Position {
                            row: primary,
                            col: secondary,
                        };
                        index += 1;
                    }
                }
                Self::Right => {
                    for primary in (0..4).rev() {
                        result[index] = Position {
                            row: secondary,
                            col: primary,
                        };
                        index += 1;
                    }
                }
                Self::Down => {
                    for primary in 0..4 {
                        result[index] = Position {
                            row: primary,
                            col: secondary,
                        };
                        index += 1;
                    }
                }
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
                *moving_state = MovingState::SetMoving;
                *moving_dir = direction;
            }
        }
    }
}

fn set_moving(
    mut commands: Commands,
    mut moving_state: ResMut<MovingState>,
    moving_dir: Res<MovingDirection>,
    mut spawn_tile_events: ResMut<Events<SpawnTileEvent>>,
    mut tiles: Query<(Entity, &Tile, &Position, &Option<Moving>, &Option<Merged>)>,
    mut merged: Query<(Entity, &Option<Merged>)>,
) {
    if *moving_state == MovingState::SetMoving {
        let mut tiles = tiles.iter();
        let mut board: [Option<(Entity, &Tile, &Position, &Option<Moving>, &Option<Merged>)>; 16] =
            Default::default();
        for tile in &mut tiles {
            let position = tile.2;
            if tile.3.is_some() {
                println!("Should not move!");
            }
            board[position.index()] = Some(tile);
        }

        let mut moving_entities = Vec::new();
        for curr_pos in moving_dir.board_iteration().iter() {
            if let Some(curr_tile) = &board[curr_pos.index()] {
                if let Some(new_pos) = moving_dir.moved_position(curr_pos) {
                    if let Some(existing_tile) = &board[new_pos.index()] {
                        if moving_entities.contains(&existing_tile.0)
                            || (curr_tile.1.level == existing_tile.1.level
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
            for (entity, merged) in &mut merged.iter() {
                if merged.is_some() {
                    commands.insert_one(entity, Option::<Merged>::None);
                }
            }

            spawn_tile_events.send(SpawnTileEvent);
            MovingState::Idle
        };
    }
}

fn moving_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut moving_state: ResMut<MovingState>,
    mut moving_anim: ResMut<Animation>,
    moving_dir: Res<MovingDirection>,
    mut animate_transform: Query<(&Position, &mut Transform, &Option<Moving>)>,
    mut update_position: Query<(Entity, &mut Position, &Option<Moving>)>,
) {
    if *moving_state == MovingState::Animating {
        if moving_anim.update(time.delta_seconds) {
            for (position, mut transform, moving) in &mut animate_transform.iter() {
                if moving.is_some() {
                    let translate: Vec3 =
                        Vec3::from(*moving_dir) * (TILE_SIZE + TILE_SPACING) * moving_anim.value();
                    transform.set_translation(Vec3::from(*position) + translate);
                }
            }
        }

        if moving_anim.finished {
            for (entity, mut position, moving) in &mut update_position.iter() {
                if moving.is_some() {
                    *position = moving_dir.moved_position(&position).unwrap();
                    commands.insert_one(entity, Option::<Moving>::None);
                }
            }

            moving_anim.reset();
            *moving_state = MovingState::Merging;
        }
    }
}

fn merging(
    mut commands: Commands,
    mut moving_state: ResMut<MovingState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tiles: Query<(Entity, &mut Tile, &Position, &mut Handle<ColorMaterial>)>,
) {
    if *moving_state == MovingState::Merging {
        let mut board = [None; 16];
        for (entity, mut tile, position, mut material) in &mut tiles.iter() {
            if let Some((existing_entity, _position)) = board[position.index()] {
                tile.level += 1;
                *material = materials.add(tile.color().into());

                commands.despawn(existing_entity);
                commands.insert_one(entity, Some(Merged));
            } else {
                board[position.index()] = Some((entity, position));
            }
        }

        *moving_state = MovingState::SetMoving;
    }
}
