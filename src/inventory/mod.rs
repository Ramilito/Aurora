mod systems;
use bevy::prelude::*;
 
use crate::components::AppState;
use self::systems::startup;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(startup.in_schedule(OnEnter(AppState::InGame)));
    }
}
