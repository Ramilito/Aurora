use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_egui::EguiPlugin;

use crate::components::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        //         register_assets(app);
        //         register_asset_loaders(app);
        //
        app.add_plugin(EguiPlugin)
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::InGame),
            )
            .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "map/tower.gltf#Scene0")]
    pub tower: Handle<Scene>,

    #[asset(path = "map/tower.gltf#Mesh0/Primitive0")]
    pub tower_mesh: Handle<Mesh>,

    #[asset(path = "map/tower.gltf")]
    pub tower_gltf: Handle<Gltf>,

    #[asset(path = "models/AlienCake/alien.gltf#Scene0")]
    pub player: Handle<Scene>,

    #[asset(path = "models/AlienCake/alien.gltf#Mesh0/Primitive0")]
    pub player_mesh: Handle<Mesh>,

    #[asset(path = "models/animated/npc_emo.gltf#Scene0")]
    pub npc_emo: Handle<Scene>,
}
