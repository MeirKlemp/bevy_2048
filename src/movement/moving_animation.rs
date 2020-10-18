//! This module contains the impl of the MovingAnimation component and
//! the impl of the Animating state's system.
use bevy::prelude::*;

use crate::components::{Animation, Position};

use super::{Moving, MovingDirection, MovingState};

/// Animating the movement of the tiles.
/// This is a global resource because all tiles
/// should be animated the same time.
pub struct MovingAnimation {
    pub animation: Animation,
}

impl Default for MovingAnimation {
    /// Sets the animation to finish after 3 updates.
    fn default() -> Self {
        Self {
            animation: Animation::new(3),
        }
    }
}

/// While the moving state is `Animating`, animating all moving tiles.
pub fn moving_animation(
    time: Res<Time>,
    mut moving_state: ResMut<MovingState>,
    mut moving_anim: ResMut<MovingAnimation>,
    moving_dir: Res<MovingDirection>,
    mut animate_transform: Query<(&Position, &mut Transform, &Option<Moving>)>,
    mut update_position: Query<(&mut Position, &mut Option<Moving>)>,
) {
    if matches!(*moving_state, MovingState::Animating) {
        // Checking if should update the transform of the tiles.
        if moving_anim.animation.update(time.delta_seconds) {
            // For each tile that is moving, update its transform.
            for (position, mut transform, moving) in &mut animate_transform.iter() {
                if moving.is_some() {
                    // The amount to move from its position.
                    let translate: Vec3 = Vec3::from(*moving_dir)
                        * (crate::TILE_SIZE + crate::TILE_SPACING)
                        * moving_anim.animation.value();

                    // update the transform.
                    transform.set_translation(Vec3::from(*position) + translate);
                }
            }
        }

        // If the animation have been finished, remove all moving and
        // update the position component.
        if moving_anim.animation.finished() {
            for (mut position, mut moving) in &mut update_position.iter() {
                if moving.is_some() {
                    *position = moving_dir.moved_position(&position).unwrap();
                    *moving = None;
                }
            }

            moving_anim.animation.reset();
            *moving_state = MovingState::Merging;
        }
    }
}
