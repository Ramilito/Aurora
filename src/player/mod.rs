use bevy::prelude::*;

pub mod components;
mod systems;

use systems::{load_assets, player_movement};

use crate::components::AppState;

use self::systems::read_result_system;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_assets.in_schedule(OnEnter(AppState::InGame)))
            .add_system(read_result_system)
            .add_system(player_movement);
    }
}
