use bevy::prelude::*;
mod components;
mod level_one;
mod level_two;
mod state_machine;
use crate::components::AppState;

use self::{
    components::LevelState,
    state_machine::{solved, unsolved},
};
pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LevelState>()
            .add_system(level_one::setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(level_one::load_assets.in_schedule(OnEnter(AppState::InGame)))
            .add_system(level_one::load_map.in_schedule(OnEnter(AppState::InGame)))
            .add_system(level_one::load_puzzle.in_schedule(OnEnter(AppState::InGame)))
            .add_systems((solved, unsolved).in_set(OnUpdate(AppState::InGame)))
            .add_system(level_two::load_map.in_schedule(OnEnter(AppState::InGame)));
    }
}
