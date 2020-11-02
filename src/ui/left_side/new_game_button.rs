use bevy::prelude::*;

use super::LeftSideNode;
use crate::components::GameState;

pub enum NewGameButtonState {
    Idle,
    Down,
    Up,
}

impl NewGameButtonState {
    pub fn update_state(&mut self) {
        match self {
            NewGameButtonState::Down => {
                *self = NewGameButtonState::Up;
            }
            NewGameButtonState::Up => {
                *self = NewGameButtonState::Idle;
            }
            NewGameButtonState::Idle => (),
        }
    }
}

pub struct NewGameButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for NewGameButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        NewGameButtonMaterials {
            normal: materials.add(Color::rgb_u8(40, 40, 40).into()),
            hovered: materials.add(Color::rgb_u8(64, 64, 64).into()),
            pressed: materials.add(Color::rgb_u8(50, 50, 200).into()),
        }
    }
}

pub fn new_game_button_system(
    mut game_state: ResMut<GameState>,
    button_materials: Res<NewGameButtonMaterials>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &mut NewGameButtonState,
    )>,
) {
    for (_, interaction, mut material, mut button_state) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed;
                *button_state = NewGameButtonState::Down;
            }
            Interaction::Hovered => {
                *material = button_materials.hovered;
                button_state.update_state();

                if matches!(*button_state, NewGameButtonState::Up) {
                    *game_state = GameState::Restarting;
                }
            }
            Interaction::None => {
                *material = button_materials.normal;
                button_state.update_state();
            }
        }
    }
}

pub fn spawn_new_game_button(
    mut commands: Commands,
    assets: Res<AssetServer>,
    button_materials: Res<NewGameButtonMaterials>,
    ls_node_entity: Entity,
    _: &LeftSideNode,
) {
    let font_handle = assets.get_handle("assets/fonts/FiraSans-Bold.ttf").unwrap();

    commands
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
            parent.spawn(TextComponents {
                style: Style::default(),
                text: Text {
                    value: "New Game".to_string(),
                    font: font_handle,
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            });
        })
        .with(NewGameButtonState::Idle);

    commands.push_children(ls_node_entity, &[commands.current_entity().unwrap()]);
}
