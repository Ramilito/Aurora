use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

#[derive(Component)]
struct PlayerCamera;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "map/tower.gltf#Scene0")]
    pub tower: Handle<Scene>,

    #[asset(path = "map/tower.gltf#Mesh0/Primitive0")]
    pub tower_mesh: Handle<Mesh>,

    #[asset(path = "models/AlienCake/alien.glb#Scene0")]
    pub player: Handle<Scene>,
}
