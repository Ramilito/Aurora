use bevy::{gltf::GltfMesh, prelude::*};
use bevy_rapier3d::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default());
    }
}
pub fn get_collider(
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    meshes: ResMut<Assets<Mesh>>,
    mesh_handle: (&String, &Handle<GltfMesh>),
) {
    let (name, gltf_mesh) = mesh_handle;
    if !name.contains("collider") {
        return;
    }

    let mesh = assets_gltfmesh.get(gltf_mesh).clone().unwrap();
    let _collider = Collider::from_bevy_mesh(
        meshes.get(&mesh.primitives[0].mesh.clone()).unwrap(),
        &ComputedColliderShape::TriMesh,
    )
    .unwrap();
}
