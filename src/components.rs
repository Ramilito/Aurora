use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::AssetCollection;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    AssetLoading,
    InGame,
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
