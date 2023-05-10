mod camera;
mod collision;
mod components;
mod game;
mod map;
mod player;
mod npc;
mod skymap;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use components::{AppState, MyAssets};

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
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::InGame),
        )
        .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(game::InGamePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(npc::NpcPlugin)
        .run();
}
