use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::KayakWidgets};

use crate::components::AppState;

use self::systems::startup;
mod systems;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_system(startup.in_schedule(OnEnter(AppState::InGame)));
    }
}
