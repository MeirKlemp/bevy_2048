use bevy::prelude::*;

use super::RightSideNode;

const EXPLANATION_TEXT: &str = r#"Use arrow keys or
WASD keys to merge
the tiles with the
same color. Press
SPACE to restart."#;

pub fn spawn_explanation_node(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rs_node_entity: Entity,
    _: &RightSideNode,
) {
    let font_handle = assets.get_handle("assets/fonts/FiraSans-Bold.ttf").unwrap();

    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(240.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::Wrap,
                margin: Rect::all(Val::Px(15.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgb_u8(40, 40, 40).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            for line in EXPLANATION_TEXT.lines().rev() {
                spawn_text(parent, line, 25.0, font_handle);
            }
            // Title.
            spawn_text(parent, "How to play:", 40.0, font_handle);
        });

    commands.push_children(rs_node_entity, &[commands.current_entity().unwrap()]);
}

fn spawn_text(parent: &mut ChildBuilder, text: &str, font_size: f32, font_handle: Handle<Font>) {
    parent
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(font_size)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextComponents {
                style: Style::default(),
                text: Text {
                    value: text.to_string(),
                    font: font_handle,
                    style: TextStyle {
                        font_size,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            });
        });
}
