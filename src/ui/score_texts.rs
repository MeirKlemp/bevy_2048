use bevy::prelude::*;

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

pub fn spawn_score_text(
    parent: &mut ChildBuilder,
    font_handle: Handle<Font>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    parent
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(230.0), Val::Px(100.0)),
                margin: Rect::all(Val::Px(15.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgb_u8(119, 110, 101).into()),
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
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                })
                .with(ScoreText);
        });
}

pub fn spawn_highscore_text(
    parent: &mut ChildBuilder,
    font_handle: Handle<Font>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    parent
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
            material: materials.add(Color::rgb_u8(119, 110, 101).into()),
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
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                })
                .with(HighScoreText);
        });
}