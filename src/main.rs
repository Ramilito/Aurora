mod camera;
mod collision;
mod components;
mod game;
mod inventory;
mod map;
mod menu;
mod npc;
mod player;
mod puzzles;
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
        .add_plugin(WorldInspectorPlugin::new())
        //.add_plugin(EguiPlugin)
        .add_plugin(StateMachinePlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(InGamePlugin)
        .add_plugin(inventory::InventoryPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(npc::NpcPlugin)
        .add_plugin(puzzles::PuzzlePlugin)
        .run();
}
