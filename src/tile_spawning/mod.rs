mod despawn_animation;
mod spawn_animation;
mod spawn_tiles;

pub use despawn_animation::DespawnAnimation;
pub use spawn_animation::SpawnAnimation;
pub use spawn_tiles::SpawnTileEvent;
pub use spawn_tiles::SpawnTileListener;

use bevy::prelude::*;
pub struct SpawnTilePlugin;

impl Plugin for SpawnTilePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<SpawnTileListener>()
            .add_event::<SpawnTileEvent>()
            .add_system(spawn_tiles::spawn_tiles.system())
            .add_system(spawn_animation::spawn_animation.system())
            .add_system(despawn_animation::despawn_animation.system());
    }
}
