mod how_to_node;
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
                how_to_node::spawn_how_to_node.system(),
            )
            .add_startup_system_to_stage(POST_RS_CREATION_STAGE, title::spawn_title.system());
    }
}

pub struct RightSideNode;

fn spawn_right_side_node(mut commands: Commands, root_entity: Entity, _: &RootNode) {
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(50.0)),
                align_self: AlignSelf::FlexEnd,
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

    commands.push_children(root_entity, &[commands.current_entity().unwrap()]);
}
