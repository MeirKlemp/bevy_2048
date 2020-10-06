mod highscore;
pub use highscore::HighScore;

pub struct Score(pub u32);

use bevy::prelude::*;
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<HighScore>()
            .add_resource(Score(0))
            .add_system(highscore::update_highscore.system());
    }
}
