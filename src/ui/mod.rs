mod score_texts;
mod new_game_button;

use bevy::prelude::*;
use new_game_button::NewGameButtonMaterials;
/// This plugin builds the ui system into the app.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .init_resource::<NewGameButtonMaterials>()
            .add_startup_system(setup.system())
            .add_system(new_game_button::new_game_button_system.system())
            .add_system(score_texts::score_text.system())
            .add_system(score_texts::highscore_text.system());
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    button_materials: Res<NewGameButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Loading the font.
    let font_handle = assets.load("assets/fonts/FiraSans-Bold.ttf").unwrap();

    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        // root node
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                // Left vertical node.
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Px(260.0), Val::Px(360.0)),
                        align_self: AlignSelf::FlexEnd,
                        align_items: AlignItems::FlexEnd,
                        flex_wrap: FlexWrap::Wrap,
                        ..Default::default()
                    },
                    // material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
                    draw: Draw {
                        is_visible: false,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    score_texts::spawn_score_text(parent, font_handle, &mut materials);
                    score_texts::spawn_highscore_text(parent, font_handle, &mut materials);
                    new_game_button::spawn_new_game_button(parent, font_handle, &button_materials)
                });
        });
}
