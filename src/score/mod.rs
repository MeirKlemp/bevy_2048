pub struct Score(pub u32);

use bevy::prelude::*;
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_resource(Score(0));
    }
}
