use bevy::prelude::*;

mod components;
mod systems;
use systems::setup;

use self::systems::load_assets;
use crate::components::AppState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_assets.in_schedule(OnEnter(AppState::InGame)))
            .add_startup_system(setup);
    }
}
