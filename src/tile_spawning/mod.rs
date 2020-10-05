mod despawn_animation;
mod spawn_animation;
mod spawn_tiles;
mod despawn_tiles;

pub use despawn_animation::DespawnAnimation;
pub use despawn_tiles::Despawn;
pub use spawn_animation::SpawnAnimation;
pub use spawn_tiles::SpawnTileEvent;
pub use spawn_tiles::SpawnTileListener;

pub static DESPAWN_STAGE: &str = "DESPAWN";
pub static SPAWN_STAGE: &str = "SPAWN";

use bevy::prelude::*;
pub struct SpawnTilePlugin;

impl Plugin for SpawnTilePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<SpawnTileListener>()
            .add_stage(DESPAWN_STAGE)
            .add_stage_after(DESPAWN_STAGE, SPAWN_STAGE)
            .add_event::<SpawnTileEvent>()
            .add_system_to_stage(SPAWN_STAGE, spawn_tiles::spawn_tiles.system())
            .add_system_to_stage(SPAWN_STAGE, spawn_animation::spawn_animation.system())
            .add_system_to_stage(DESPAWN_STAGE, despawn_tiles::despawn_tiles.system())
            .add_system_to_stage(DESPAWN_STAGE, despawn_animation::despawn_animation.system());
    }
}
