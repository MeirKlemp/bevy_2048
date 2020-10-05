use bevy::prelude::*;

use crate::{
    components::{Position, Tile},
    movement::{Merged, Moving},
};

use super::DespawnAnimation;

pub struct Despawn;

pub fn despawn_tiles(mut commands: Commands, entity: Entity, _: &Despawn) {
    commands.remove_one::<Tile>(entity);
    commands.remove_one::<Position>(entity);
    commands.remove_one::<Despawn>(entity);
    commands.remove_one::<Option<Moving>>(entity);
    commands.remove_one::<Option<Merged>>(entity);
    commands.insert_one(entity, DespawnAnimation::default());
}
