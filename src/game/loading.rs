use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_gltf_collider::get_scene_colliders;
use bevy_rapier3d::prelude::Collider;

use crate::components::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::AssetLoaded),
        )
        .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading)
        .add_system(add_colliders.in_schedule(OnEnter(AppState::AssetLoaded)));
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
    pub tower_colliders: Vec<(Collider, Transform)>,

    #[asset(path = "models/AlienCake/alien.gltf#Scene0")]
    pub player: Handle<Scene>,
    #[asset(path = "models/AlienCake/alien.gltf")]
    pub player_gltf: Handle<Gltf>,
    #[asset(path = "models/AlienCake/alien.gltf#Mesh0/Primitive0")]
    pub player_mesh: Handle<Mesh>,
    pub player_colliders: Vec<(Collider, Transform)>,

    #[asset(path = "models/animated/npc_emo.gltf#Scene0")]
    pub npc_emo: Handle<Scene>,
    pub npc_colliders: Vec<(Collider, Transform)>,

    #[asset(path = "models/platform.gltf#Scene0")]
    pub platform: Handle<Scene>,
    pub platform_colliders: Vec<(Collider, Transform)>,

    #[asset(path = "models/barrel.gltf#Scene0")]
    pub barrel: Handle<Scene>,
    pub barrel_colliders: Vec<(Collider, Transform)>,

    #[asset(path = "models/sword.gltf#Scene0")]
    pub sword: Handle<Scene>,
    pub sword_colliders: Vec<(Collider, Transform)>,
}

fn add_colliders(
    mut meshes: ResMut<Assets<Mesh>>,
    mut _my_assets: ResMut<MyAssets>,
    mut app_state: ResMut<NextState<AppState>>,
    mut scenes: ResMut<Assets<Scene>>,
) {
    _my_assets.platform_colliders = get_scene_colliders(
        &mut meshes,
        &mut scenes.get_mut(&_my_assets.platform).unwrap().world,
    )
    .unwrap();

    _my_assets.player_colliders = get_scene_colliders(
        &mut meshes,
        &mut scenes.get_mut(&_my_assets.player).unwrap().world,
    )
    .unwrap();

    _my_assets.barrel_colliders = get_scene_colliders(
        &mut meshes,
        &mut scenes.get_mut(&_my_assets.barrel).unwrap().world,
    )
    .unwrap();

    _my_assets.sword_colliders = get_scene_colliders(
        &mut meshes,
        &mut scenes.get_mut(&_my_assets.sword).unwrap().world,
    )
    .unwrap();

    app_state.set(AppState::InMenu);
}
use bevy::gltf::Gltf;
