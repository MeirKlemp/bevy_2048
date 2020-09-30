use bevy::{prelude::*, render::pass::ClearColor};
use rand::Rng;

const BOARD_SIZE: f32 = 500.0;
const TILE_SIZE: f32 = (BOARD_SIZE * 0.85) / 4.0;
const TILE_SPACING: f32 = (BOARD_SIZE * 0.15) / 5.0;

const STARTING_TILES: u32 = 2;
const MAX_STARTING_LEVEL: u32 = 2; // [0, MAX_STARTING_LEVEL(exluding))

fn main() {
    App::build()
        .add_default_plugins()
        .add_event::<SpawnTileEvent>()
        .init_resource::<SpawnTileListener>()
        // Set background color.
        .add_resource(ClearColor(Color::rgb_u8(250, 248, 239)))
        .add_startup_system(setup.system())
        .add_system(spawn_tiles.system())
        .add_system(spawn_animation.system())
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

    let offset = Vec3::new(
        -(BOARD_SIZE - TILE_SIZE) / 2.0 + TILE_SPACING,
        -(BOARD_SIZE - TILE_SIZE) / 2.0 + TILE_SPACING,
        0.0,
    );

    for row in 0..4 {
        let y_pos = (TILE_SIZE + TILE_SPACING) * row as f32;
        for col in 0..4 {
            let x_pos = (TILE_SIZE + TILE_SPACING) * col as f32;
            let position = Vec3::new(x_pos, y_pos, 0.0) + offset;

            commands.spawn(SpriteComponents {
                material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
                sprite: Sprite::new(Vec2::new(TILE_SIZE, TILE_SIZE)),
                transform: Transform::from_translation(position),
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
#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl From<Position> for Vec3 {
    /// Transforms a position into a world point.
    fn from(pos: Position) -> Self {
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

/// Component for animating the spawning of a new tile.
struct SpawnAnimation {
    timer: Timer,
    ticks: usize,
    max_ticks: usize,
    finished: bool,
}

impl SpawnAnimation {
    /// Returns a value in the range [0, 1] for the animation.
    fn value(&self) -> f32 {
        self.ticks as f32 / self.max_ticks as f32
    }

    /// Updates the animation, needs delta_seconds from a timer.
    /// Returns `true` if the animation is not finished.
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
}

impl Default for SpawnAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / 60.0, true),
            ticks: 0,
            max_ticks: 3,
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
    mut animation: Mut<SpawnAnimation>,
    mut sprite: Mut<Sprite>,
) {
    if animation.update(time.delta_seconds) {
        // Updating the sprite size while the animation is not finished.
        let size = TILE_SIZE * animation.value();
        sprite.size.set_x(size);
        sprite.size.set_y(size);
    } else {
        // When the animation is finished, the component is being removed.
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

            // Spawning a new tile.
            commands
                .spawn(SpriteComponents {
                    material: materials.add(tile.color().into()),
                    transform: Transform::from_translation(pos.into()),
                    ..Default::default()
                })
                .with(tile)
                .with(pos)
                .with(SpawnAnimation::default());
        } else {
            #[cfg(debug_assertions)]
            panic!("spawn_tiles(): Tried to spawn a tile when the board was full.")
        }
    }
}
