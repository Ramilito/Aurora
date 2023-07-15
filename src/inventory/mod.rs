mod systems;
use bevy::prelude::*;
use kayak_ui::{prelude::KayakContextPlugin, widgets::KayakWidgets};

use self::systems::startup;
use crate::components::AppState;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_systems(OnEnter(AppState::InGame), startup);
    }
}
