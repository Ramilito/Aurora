use bevy::prelude::*;

mod compontents;
mod systems;
use systems::setup_game;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game);
    }
}
