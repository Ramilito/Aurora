use bevy::prelude::*;
mod components;
mod level_one;
mod level_two;
// mod state_machine;
use crate::components::AppState;

use self::{
    components::LevelState,
    // state_machine::{solved, unsolved},
};
pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LevelState>()
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    level_one::setup,
                    level_one::load_assets,
                    level_one::load_map,
                    level_one::load_puzzle,
                ),
            )
            // .add_systems(Update, (solved, unsolved).in_set(AppState::InGame))
            .add_systems(OnEnter(AppState::InGame), level_two::load_map);
    }
}
