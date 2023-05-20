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
        app.add_system(load_assets.in_schedule(OnEnter(AppState::InGame)))
            .add_startup_system(setup)
            .add_system(setup_scene_once_loaded)
            .add_systems((idle, indialog).in_set(OnUpdate(AppState::InGame)));
    }
}
