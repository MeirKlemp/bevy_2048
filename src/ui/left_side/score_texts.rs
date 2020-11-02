use bevy::prelude::*;

use super::LeftSideNode;
use crate::{score::HighScore, score::Score};

pub struct ScoreText;

pub struct HighScoreText;

/// Updating the score text according to the score.
pub fn score_text(score: Res<Score>, mut text: Mut<Text>, _: &ScoreText) {
    text.value = format!("Score: {}", score.0)
}

/// Updating the score text according to the score.
pub fn highscore_text(highscore: Res<HighScore>, mut text: Mut<Text>, _: &HighScoreText) {
    text.value = format!("Best: {}", highscore.0)
}

pub fn spawn_texts(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ls_node_entity: Entity,
    _: &LeftSideNode,
) {
    let font_handle = assets.get_handle("assets/fonts/FiraSans-Bold.ttf").unwrap();

    // Spawning score text.
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(230.0), Val::Px(100.0)),
                margin: Rect::all(Val::Px(15.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgb_u8(40, 40, 40).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // Score Text.
            parent
                .spawn(TextComponents {
                    style: Style::default(),
                    text: Text {
                        value: "Score: 0".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                })
                .with(ScoreText);
        });
    let score_entity = commands.current_entity().unwrap();

    // Spawning highscore text.
    commands
        .spawn(NodeComponents {
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
            material: materials.add(Color::rgb_u8(40, 40, 40).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // Score Text.
            parent
                .spawn(TextComponents {
                    style: Style::default(),
                    text: Text {
                        value: "Best: 0".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                })
                .with(HighScoreText);
        });
    let highscore_entity = commands.current_entity().unwrap();

    commands.push_children(ls_node_entity, &[score_entity, highscore_entity]);
}
