use bevy::{prelude::*, render::pass::ClearColor};

const BOARD_SIZE: f32 = 500.0;
const CELL_SIZE: f32 = (BOARD_SIZE * 0.85) / 4.0;
const CELL_SPACING: f32 = (BOARD_SIZE * 0.15) / 5.0;

fn main() {
    App::build()
        .add_default_plugins()
        // Set background color.
        .add_resource(ClearColor(Color::rgb_u8(250, 248, 239)))
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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
}
