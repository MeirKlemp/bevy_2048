use bevy::prelude::*;
use std::convert::TryFrom;

use super::{MovingDirection, MovingState};

/// While the moving state is `Idle`, getting the input
/// of the user.
/// If the user pressed the arrows or a,w,d,s keys,
/// the direction is being chosen
pub fn moving_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut moving_state: ResMut<MovingState>,
    mut moving_dir: ResMut<MovingDirection>,
) {
    if *moving_state == MovingState::Idle {
        // Iterating through the keys that were just pressed by the user.
        for key in keyboard_input.get_just_pressed() {
            // Checking if the keys can be converted into a direction
            if let Ok(direction) = MovingDirection::try_from(key) {
                // Setting the direction.
                *moving_dir = direction;
                // Setting the moving state to `SetMoving` with starting.
                *moving_state = MovingState::SetMoving { starting: true };
            }
        }
    }
}
