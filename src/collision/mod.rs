use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ));
    }
}

// pub fn get_scene_colliders(
//     asset_gltf: &Res<Assets<Gltf>>,
//     asset: &Handle<Gltf>,
//     assets_gltfmesh: &Res<Assets<GltfMesh>>,
//     meshes: &Res<Assets<Mesh>>,
//     world: &mut World,
// ) -> Vec<(Collider, Transform)> {
//     let mut result = Vec::new();
//     let mut entities_to_despawn = Vec::new();
//
//     if let Some(player) = asset_gltf.get(&asset) {
//         for mesh_handle in player.named_meshes.iter() {
//             if !mesh_handle.0.contains("colmesh") {
//                 continue;
//             }
//
//             let mut meshes_q = world.query::<(Entity, &Name, &GltfExtras)>();
//
//             for (entity, entity_name, _extra) in meshes_q.iter(world) {
//                 if !entity_name.contains("colonly") {
//                     continue;
//                 }
//
//                 let t = *world.get::<Transform>(entity).unwrap();
//
//                 let (_name, gltf_mesh) = mesh_handle;
//                 let mesh = assets_gltfmesh.get(gltf_mesh).clone().unwrap();
//
//                 let collider = Collider::from_bevy_mesh(
//                     meshes.get(&mesh.primitives[0].mesh.clone()).unwrap(),
//                     &ComputedColliderShape::TriMesh,
//                 )
//                 .unwrap();
//                 println!("entity_name: {:?}", entity_name);
//                 println!("t: {:?}", t);
//
//                 result.push((
//                     collider,
//                     // t
//                     Transform {
//                         translation: t.translation,
//                         rotation: t.rotation,
//                         // rotation: Quat::from_euler(EulerRot::XYZ, t.rotation.x.to_radians(), t.rotation.y.to_radians(), t.rotation.z.to_radians()),
//                         // rotation: Quat {
//                         //     x: t.rotation.x,
//                         //     y: t.rotation.y,
//                         //     z: t.rotation.z,
//                         //     w: t.rotation.w,
//                         // },
//                         scale: t.scale,
//                     },
//                 ));
//                 entities_to_despawn.push(entity);
//             }
//         }
//         for e in entities_to_despawn {
//             despawn_with_children_recursive(world, e);
//         }
//     }
//
//     return result;
// }
