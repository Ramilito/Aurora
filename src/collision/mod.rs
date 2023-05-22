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
) -> Vec<(Collider, Transform)> {
    let mut result = Vec::new();
    if let Some(platform) = asset_gltf.get(&_my_assets.platform_gltf) {
        for mesh_handle in platform.named_meshes.iter() {
            let (_name, gltf_mesh) = mesh_handle;

            let mesh = assets_gltfmesh.get(gltf_mesh).clone().unwrap();
            let collider = Collider::from_bevy_mesh(
                meshes.get(&mesh.primitives[0].mesh.clone()).unwrap(),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap();

            let transform = Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                (-90.0_f32).to_radians(),
                (0.0_f32).to_radians(),
                (0.0_f32).to_radians(),
            ));

            result.push((collider, transform));
        }
    }
    return result;
}
