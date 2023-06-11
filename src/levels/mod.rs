use bevy::prelude::*;
mod components;
mod state_machine;
mod systems;

use crate::components::AppState;

use self::{
    state_machine::{solved, unsolved},
    systems::{load_assets, setup},
};
pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(load_assets.in_schedule(OnEnter(AppState::InGame)))
            .add_systems((solved, unsolved).in_set(OnUpdate(AppState::InGame)));
    }
}
