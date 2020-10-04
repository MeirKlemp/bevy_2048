use crate::components::Animation;
use bevy::prelude::*;

/// Component used to animate the tiles despawning.
pub struct DespawnAnimation {
    pub animation: Animation,
}

impl Default for DespawnAnimation {
    /// Sets the animation to finish after 3 updates.
    fn default() -> Self {
        Self {
            animation: Animation::new(3),
        }
    }
}

/// Despawning with an animation all tiles that have a despawn animation.
pub fn despawn_animation(
    mut commands: Commands,
    time: Res<Time>,
    entity: Entity,
    mut despawn_anim: Mut<DespawnAnimation>,
    mut sprite: Mut<Sprite>,
) {
    if despawn_anim.animation.update(time.delta_seconds) {
        // Updating the sprite size while the animation is not finished.
        let size = crate::TILE_SIZE * despawn_anim.animation.rev_value();
        sprite.size.set_x(size);
        sprite.size.set_y(size);
    }

    // When the animation is finished, the entity will be despawned.
    if despawn_anim.animation.finished() {
        commands.despawn(entity);
    }
}
