use bevy::prelude::*;

mod components;
mod npc_emo;
mod state_machine;

use self::npc_emo::{load_assets, setup, setup_scene_once_loaded};
use self::state_machine::{idle, indialog};
use crate::components::AppState;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), load_assets)
            .add_systems(Startup, setup)
            .add_systems(Update, setup_scene_once_loaded)
            .add_systems(Update, (idle, indialog).in_set(AppState::InGame));
    }
}
