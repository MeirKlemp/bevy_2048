mod finish_moving;
mod merge_animation;
mod merging;
mod moving_animation;
mod moving_direction;
mod moving_input;
mod moving_state;
mod set_moving;

pub use merge_animation::MergeAnimation;
pub use moving_animation::MovingAnimation;
pub use moving_direction::MovingDirection;
pub use moving_state::MovingState;
/// Component to tell if a tile is moving or not.
pub struct Moving;

/// Component to tell if a tile has been merged or not.
pub struct Merged;

use bevy::prelude::*;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<MovingAnimation>()
            .init_resource::<MovingState>()
            .add_resource(MovingDirection::Left)
            .add_system(moving_input::moving_input.system())
            .add_system(set_moving::set_moving.system())
            .add_system(moving_animation::moving_animation.system())
            .add_system(merging::merging.system())
            .add_system(merge_animation::merge_animation.system())
            .add_system(finish_moving::finish_moving.system());
    }
}
