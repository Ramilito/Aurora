use bevy::prelude::*;
// mod compontents;
// mod resources;
// mod puzzles;
mod systems;

use crate::components::AppState;

use self::systems::{check_puzzle_both, check_puzzle_left, check_puzzle_right, setup};
pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(setup.in_schedule(OnEnter(AppState::InGamme)));
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(check_puzzle_right.in_set(OnUpdate(AppState::InGame)))
            .add_system(check_puzzle_left.in_set(OnUpdate(AppState::InGame)))
            .add_system(check_puzzle_both.in_set(OnUpdate(AppState::InGame)));
    }
}
