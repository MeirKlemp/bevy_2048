use bevy::prelude::*;

pub struct NewGameButton;

pub struct NewGameButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for NewGameButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        NewGameButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

pub fn new_game_button_system(
    button_materials: Res<NewGameButtonMaterials>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        // &NewGameButton,
    )>,
) {
    for (_button, interaction, mut material) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed;
            }
            Interaction::Hovered => {
                *material = button_materials.hovered;
            }
            Interaction::None => {
                *material = button_materials.normal;
            }
        }
    }
}

pub fn spawn_new_game_button(
    parent: &mut ChildBuilder,
    font_handle: Handle<Font>,
    button_materials: &Res<NewGameButtonMaterials>,
) {
    parent
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(230.0), Val::Px(100.0)),
                margin: Rect {
                    left: Val::Px(15.0),
                    top: Val::Px(15.0),
                    right: Val::Px(15.0),
                    bottom: Val::Px(0.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal,
            ..Default::default()
        })
        .with_children(|parent| {
            // Score Text.
            parent
                .spawn(TextComponents {
                    style: Style::default(),
                    text: Text {
                        value: "New Game".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                })
                .with(NewGameButton);
        });
}