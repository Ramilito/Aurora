use crate::components::AppState;
use bevy::prelude::*;

pub struct InGamePlugin;

use bevy::gltf::{Gltf, GltfMesh};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, Dominance, Restitution, RigidBody};

use super::loading::MyAssets;


impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_assets.in_schedule(OnEnter(AppState::InGame)))
            .add_startup_system(setup);
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("map/clouds_skybox.glb#Scene0"),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        point_light: PointLight {
            intensity: 1000.0,
            color: Color::ORANGE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-5.0, 25.0, 10.0),
        point_light: PointLight {
            intensity: 10000.0,
            range: 1000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });
}

pub fn load_assets(
    _my_assets: Res<MyAssets>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    asset_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/sword.gltf#Scene0"),
            ..default()
        })
        // .insert(Collider::cuboid(0.25, 0.4, 0.2))
        // .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 2.5, 0.5)));
    load_map(_my_assets, commands, meshes, asset_gltf, assets_gltfmesh);
}

fn load_map(
    _my_assets: Res<MyAssets>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    asset_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    if let Some(tower) = asset_gltf.get(&_my_assets.tower_gltf) {
        let commons = TransformBundle::from(
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5)).with_rotation(
                Quat::from_euler(
                    EulerRot::XYZ,
                    (-90.0_f32).to_radians(),
                    (0.0_f32).to_radians(),
                    (0.0_f32).to_radians(),
                ),
            ),
        );
        commands
            .spawn(SceneBundle {
                scene: tower.scenes[0].clone(),
                ..default()
            })
            .insert(commons);
        commands
            .spawn(RigidBody::Fixed)
            .insert(Restitution::coefficient(0.0))
            .insert(Dominance::group(1))
            .insert(commons)
            .with_children(|children| {
                for mesh_handle in tower.named_meshes.iter() {
                    let (name, gltf_mesh) = mesh_handle;
                    if !name.contains("collider") {
                        continue;
                    }

                    let mesh = assets_gltfmesh.get(gltf_mesh).clone().unwrap();
                    let collider = Collider::from_bevy_mesh(
                        meshes.get(&mesh.primitives[0].mesh.clone()).unwrap(),
                        &ComputedColliderShape::TriMesh,
                    )
                    .unwrap();

                    children
                        .spawn(collider)
                        .insert(Dominance::group(1))
                        .insert(Restitution::coefficient(0.0))
                        .insert(TransformBundle::from(Transform::default()));
                }
                children.spawn(RigidBody::Fixed);
            });
    }
}
