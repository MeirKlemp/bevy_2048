use bevy::{prelude::*, render::pass::ClearColor};
use rand::Rng;

const BOARD_SIZE: f32 = 500.0;
const CELL_SIZE: f32 = (BOARD_SIZE * 0.85) / 4.0;
const CELL_SPACING: f32 = (BOARD_SIZE * 0.15) / 5.0;

const STARTING_CELLS: u32 = 2;
const MAX_STARTING_LEVEL: u32 = 2; // [0, MAX_STARTING_LEVEL(exluding))

fn main() {
    App::build()
        .add_default_plugins()
        .add_event::<SpawnCellEvent>()
        .init_resource::<SpawnCellListener>()
        // Set background color.
        .add_resource(ClearColor(Color::rgb_u8(250, 248, 239)))
        .add_startup_system(setup.system())
        .add_system(spawn_cells.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_cell_events: ResMut<Events<SpawnCellEvent>>,
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

    // Creating a grid of empty cells.
    let offset = Vec3::new(
        -(BOARD_SIZE - CELL_SIZE) / 2.0 + CELL_SPACING,
        -(BOARD_SIZE - CELL_SIZE) / 2.0 + CELL_SPACING,
        0.0,
    );

    for row in 0..4 {
        let y_pos = (CELL_SIZE + CELL_SPACING) * row as f32;
        for col in 0..4 {
            let x_pos = (CELL_SIZE + CELL_SPACING) * col as f32;
            let position = Vec3::new(x_pos, y_pos, 0.0) + offset;

            commands.spawn(SpriteComponents {
                material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
                sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                transform: Transform::from_translation(position),
                ..Default::default()
            });
        }
    }

    // Spawning 2 cells at the beginning.
    for _ in 0..STARTING_CELLS {
        spawn_cell_events.send(SpawnCellEvent);
    }
}

#[derive(Debug)]
struct Cell {
    level: u32,
}

impl Cell {
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

    fn score(&self) -> u32 {
        2u32.pow(self.level + 1)
    }
}

struct SpawnCellEvent;

#[derive(Default)]
struct SpawnCellListener {
    reader: EventReader<SpawnCellEvent>,
}

#[derive(Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn vec3(&self) -> Vec3 {
        let offset = Vec3::new(
            -(BOARD_SIZE - CELL_SIZE) / 2.0 + CELL_SPACING,
            -(BOARD_SIZE - CELL_SIZE) / 2.0 + CELL_SPACING,
            0.0,
        );

        Vec3::new(
            (CELL_SIZE + CELL_SPACING) * self.col as f32,
            (CELL_SIZE + CELL_SPACING) * self.row as f32,
            0.0,
        ) + offset
    }
}

fn spawn_cells(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut listener: ResMut<SpawnCellListener>,
    spawn_events: Res<Events<SpawnCellEvent>>,
    mut positions: Query<&Position>,
) {
    let mut free_pos = None;
    for _ in listener.reader.iter(&spawn_events) {
        if free_pos.is_none() {
            let mut vec = Vec::new();
            for row in 0..4 {
                for col in 0..4 {
                    vec.push(Position { row, col });
                }
            }

            for pos in &mut positions.iter() {
                if let Some(idx) = vec.iter().position(|x| *x == *pos) {
                    vec.remove(idx);
                }
            }

            free_pos = Some(vec);
        }

        let vec = free_pos.as_mut().unwrap();

        if vec.len() != 0 {
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0, vec.len());

            let pos = vec.remove(idx);
            let cell = Cell {
                level: rng.gen_range(0, MAX_STARTING_LEVEL),
            };

            commands
                .spawn(SpriteComponents {
                    material: materials.add(cell.color().into()),
                    transform: Transform::from_translation(pos.vec3()),
                    sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..Default::default()
                })
                .with(cell)
                .with(pos);
        } else {
            #[cfg(debug_assertions)]
            panic!("Tried to spawn a cell when the board was full.")
        }
    }
}
