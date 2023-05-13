use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, Dominance, Restitution, RigidBody};

use crate::components::MyAssets;

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
        transform: Transform::from_xyz(-5.0, 15.0, 10.0),
        point_light: PointLight {
            intensity: 10000.0,
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
) {
    if let Some(tower) = meshes.get(&_my_assets.tower_mesh) {
        let m = Collider::from_bevy_mesh(tower, &ComputedColliderShape::TriMesh).unwrap();
        commands
            .spawn(SceneBundle {
                scene: _my_assets.tower.clone(),
                ..default()
            })
            .insert(m)
            .insert(RigidBody::Fixed)
            .insert(Restitution::coefficient(0.0))
            .insert(Dominance::group(0))
            .insert(TransformBundle::from(
                Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_scale(Vec3::new(0.5, 0.5, 0.5))
                    .with_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        (-90.0_f32).to_radians(),
                        (0.0_f32).to_radians(),
                        (0.0_f32).to_radians(),
                    )),
            ));

        commands
            .spawn(SceneBundle {
                scene: asset_server.load("models/sword.gltf#Scene0"),
                ..default()
            })
            // .insert(Collider::cuboid(0.25, 0.4, 0.2))
            // .insert(RigidBody::Dynamic)
            .insert(TransformBundle::from(
                Transform::from_xyz(0.0, 2.5, 0.0), // .with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ));
    }
}
