use bevy::prelude::*;

pub mod components;
mod systems;

use crate::components::AppState;
use systems::{load_assets, player_movement};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_assets.in_schedule(OnEnter(AppState::InGame)))
            .add_system(player_movement);
    }
}
