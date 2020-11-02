mod explanation_node;
mod title;

use bevy::prelude::*;

use super::{RootNode, POST_ROOT_CREATION_STAGE};

pub struct RightSidePlugin;

static POST_RS_CREATION_STAGE: &str = "POST-RIGHT-SIDE-CREATION";

impl Plugin for RightSidePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_startup_stage_after(POST_ROOT_CREATION_STAGE, POST_RS_CREATION_STAGE)
            .add_startup_system_to_stage(POST_ROOT_CREATION_STAGE, spawn_right_side_node.system())
            .add_startup_system_to_stage(
                POST_RS_CREATION_STAGE,
                explanation_node::spawn_explanation_node.system(),
            )
            .add_startup_system_to_stage(POST_RS_CREATION_STAGE, title::spawn_title.system());
    }
}

pub struct RightSideNode;

fn spawn_right_side_node(mut commands: Commands, root_entity: Entity, _: &RootNode) {
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
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
            parent
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(345.0)),
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
                .with(RightSideNode);
        });

    commands.push_children(root_entity, &[commands.current_entity().unwrap()]);
}
