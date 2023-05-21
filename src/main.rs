mod camera;
mod collision;
mod components;
mod game;
mod map;
mod npc;
mod player;
mod puzzles;
mod skymap;

use bevy::prelude::*;
use components::AppState;
use game::{ingame::InGamePlugin, loading::LoadingPlugin};
use seldom_state::StateMachinePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Iso Staggered Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<AppState>()
        .add_plugin(StateMachinePlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(InGamePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(npc::NpcPlugin)
        .add_plugin(puzzles::PuzzlePlugin)
        .run();
}
