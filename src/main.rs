mod camera;
mod collision;
mod components;
mod game;
// mod inventory;
mod levels;
mod map;
mod menu;
mod npc;
mod player;
mod skymap;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::AppState;
use game::{ingame::InGamePlugin, loading::LoadingPlugin};
use seldom_state::StateMachinePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Aurora's Adventures"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<AppState>()
        .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(EguiPlugin)
        .add_plugins(StateMachinePlugin)
        .add_plugins((
            LoadingPlugin,
            menu::MenuPlugin,
            collision::CollisionPlugin,
            camera::CameraPlugin,
            InGamePlugin,
            player::PlayerPlugin,
            npc::NpcPlugin,
            levels::LevelsPlugin,
        ))
        // .add_plugin(inventory::InventoryPlugin)
        .run();
}
