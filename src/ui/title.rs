use bevy::prelude::*;

pub fn spawn_title(
    parent: &mut ChildBuilder,
    font_handle: Handle<Font>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    parent
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(60.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                flex_wrap: FlexWrap::Wrap,
                margin: Rect {
                    left: Val::Px(15.0),
                    top: Val::Px(15.0),
                    right: Val::Px(15.0),
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
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            });
        });
}
