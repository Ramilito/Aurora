use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_rapier3d::prelude::Collider;

use crate::{collision::get_scene_colliders, components::AppState};

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

    #[asset(path = "models/AlienCake/alien.gltf#Scene0")]
    pub player: Handle<Scene>,

    #[asset(path = "models/AlienCake/alien.gltf#Mesh0/Primitive0")]
    pub player_mesh: Handle<Mesh>,

    #[asset(path = "models/animated/npc_emo.gltf#Scene0")]
    pub npc_emo: Handle<Scene>,

    #[asset(path = "models/platform.gltf#Scene0")]
    pub platform: Handle<Scene>,

    #[asset(path = "models/platform.gltf")]
    pub platform_gltf: Handle<Gltf>,

    pub platform_colliders: Vec<(Collider, Transform)>,
}

fn add_colliders(
    meshes: Res<Assets<Mesh>>,
    mut _my_assets: ResMut<MyAssets>,
    asset_gltf: Res<Assets<Gltf>>,
    mut app_state: ResMut<NextState<AppState>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    _my_assets.platform_colliders =
        get_scene_colliders(&asset_gltf, &_my_assets, &assets_gltfmesh, &meshes);

    app_state.set(AppState::InGame);
}

// fn add_colliders(
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut scenes: ResMut<Assets<Scene>>,
//     mut _my_assets: ResMut<MyAssets>,
//     mut app_state: ResMut<NextState<AppState>>,
// ) {
//     let scene = scenes.get_mut(&_my_assets.platform).unwrap();
//
//     _my_assets.platform_colliders = get_scene_colliders(&mut meshes, &mut scene.world)
//         .expect("Failed to get colliders from scene");
//
//     app_state.set(AppState::InGame);
// }

// fn get_scene_colliders(
//     meshes: &mut Assets<Mesh>,
//     world: &mut World,
// ) -> Result<Vec<(Collider, Transform)>, ColliderFromSceneError> {
//     let mut result = Vec::new();
//     // let mut entities_to_despawn = Vec::new();
//     let mut meshes_q = world.query::<(Entity, &Name, Option<&Children>)>();
//     for (entity, entity_name, children) in meshes_q.iter(world) {
//         println!("entity:{:?}", entity);
//         println!("entity_name:{:?}", entity_name);
//         println!("children:{:?}", children);
//
//         let collider = Collider::from_bevy_mesh(
//             meshes.get(&mesh.primitives[0].mesh.clone()).unwrap(),
//             &ComputedColliderShape::TriMesh,
//         )
//         .unwrap();
//     }
//
//     // return Err(ColliderFromSceneError::NoCollidersFound);
//
//     Ok(result)
// }
