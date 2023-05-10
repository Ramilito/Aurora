use bevy::prelude::*;

pub mod components;
mod systems;

use systems::load_assets;

use crate::components::AppState;

use self::systems::{setup_scene_once_loaded, setup};

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_assets.in_schedule(OnEnter(AppState::InGame)))
            .add_startup_system(setup)
            .add_system(setup_scene_once_loaded);
    }
}
