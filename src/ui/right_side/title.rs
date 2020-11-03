use bevy::prelude::*;

use super::RightSideNode;

pub fn spawn_title(
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
                size: Size::new(Val::Percent(90.0), Val::Percent(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::Wrap,
                margin: Rect {
                    left: Val::Percent(5.0),
                    top: Val::Percent(5.0),
                    right: Val::Percent(5.0),
                    bottom: Val::Px(0.0),
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb_u8(40, 40, 40).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextComponents {
                style: Style::default(),
                text: Text {
                    value: "Bevy 2048".to_string(),
                    font: font_handle,
                    style: TextStyle {
                        font_size: 55.0,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            });
        });

    commands.push_children(rs_node_entity, &[commands.current_entity().unwrap()]);
}
