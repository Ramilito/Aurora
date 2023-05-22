use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::game::loading::MyAssets;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default());
    }
}

pub fn get_scene_colliders(
    asset_gltf: &Res<Assets<Gltf>>,
    mut _my_assets: &ResMut<MyAssets>,
    assets_gltfmesh: &Res<Assets<GltfMesh>>,
    meshes: &Res<Assets<Mesh>>,
    world: &mut World,
) -> Vec<(Collider, Transform)> {
    let mut result = Vec::new();

    if let Some(platform) = asset_gltf.get(&_my_assets.platform_gltf) {
        for mesh_handle in platform.named_meshes.iter() {
            let mut meshes_q = world.query::<(Entity, &Name)>();

            for (entity, entity_name) in meshes_q.iter(world) {
                if !entity_name.contains("collider") {
                    continue;
                }
                let transform = *world.get::<Transform>(entity).unwrap();
                let (_name, gltf_mesh) = mesh_handle;
                let mesh = assets_gltfmesh.get(gltf_mesh).clone().unwrap();

                let collider = Collider::from_bevy_mesh(
                    meshes.get(&mesh.primitives[0].mesh.clone()).unwrap(),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap();

                result.push((collider, transform));
            }
        }
    }
    return result;
}
