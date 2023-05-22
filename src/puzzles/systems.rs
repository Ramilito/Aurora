use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::game::loading::MyAssets;

#[derive(Component)]
pub struct PuzzleBoxLeft;

#[derive(Component)]
pub struct PuzzlePlateLeft;

#[derive(Component)]
pub struct PuzzlePlateRight;

#[derive(Component)]
pub struct PuzzleBoxRight;

#[derive(Bundle)]
pub struct PuzzleBundle {
    pub puzzle_box_left: PuzzleBoxLeft,
    pub puzzle_box_rigth: PuzzleBoxRight,
}

pub fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands
        .spawn((
            PuzzleBoxLeft,
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::ALICE_BLUE.into()),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.3, 0.3, 0.3))
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 3.0, 14.0)))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0));

    commands
        .spawn((
            PuzzleBoxRight,
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::DARK_GREEN.into()),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.3, 0.3, 0.3))
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 3.0, 14.0)))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0));
}

pub fn load_assets(
    _my_assets: Res<MyAssets>,
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    asset_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    if let Some(platform) = asset_gltf.get(&_my_assets.platform_gltf) {
        let commons = TransformBundle::from(
            Transform::from_xyz(2.0, 0.615, 3.0)
                .with_scale(Vec3::new(1.5, 1.5, 1.5))
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    (-90.0_f32).to_radians(),
                    (0.0_f32).to_radians(),
                    (0.0_f32).to_radians(),
                )),
        );
        commands
            .spawn((
                PuzzlePlateRight,
                SceneBundle {
                    scene: asset_server.load("models/platform.gltf#Scene0"),
                    ..default()
                },
            ))
            .insert(commons);

        commands.spawn(RigidBody::Fixed).insert(commons).with_children(|children| {
            for mesh_handle in platform.named_meshes.iter() {
                let (_name, gltf_mesh) = mesh_handle;

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
        // .with_children(|children| {
        //     for mesh_handle in tower.named_meshes.iter() {
        //         let collider = get_collider(assets_gltfmesh, meshes, mesh_handle);
        //
        //         children
        //             .spawn(collider)
        //             .insert(Dominance::group(1))
        //             .insert(Restitution::coefficient(0.0))
        //             .insert(TransformBundle::from(Transform::default()));
        //     }
        // });
    }
}

pub fn check_puzzle_right(
    query_plate: Query<&Transform, With<PuzzlePlateRight>>,
    query_box: Query<&Transform, With<PuzzleBoxRight>>,
    mut commands: Commands,
    mut done: Local<bool>,
) {
    let _box = query_box.single();
    if let Ok(_plate) = query_plate.get_single() {
        let distance = _plate.translation.distance(_box.translation);

        if distance < 1.0 {
            println!("check puzzle right");
            if !*done {
                commands.spawn(PointLightBundle {
                    transform: Transform::from_xyz(2.0, 0.715, 3.0),
                    point_light: PointLight {
                        intensity: 1000.0,
                        color: Color::ORANGE,
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                });
                *done = true;
            }
        } else {
            *done = false;
        }
    }
}

pub fn check_puzzle_left(
    query_plate: Query<&Transform, With<PuzzlePlateLeft>>,
    query_box: Query<&Transform, With<PuzzleBoxLeft>>,
    mut commands: Commands,
    mut done: Local<bool>,
) {
    let _box = query_box.single();
    if let Ok(_plate) = query_plate.get_single() {
        let distance = _plate.translation.distance(_box.translation);

        if distance < 1.0 {
            println!("check puzzle left");
            if !*done {
                commands.spawn(PointLightBundle {
                    transform: Transform::from_xyz(2.0, 0.715, 3.0),
                    point_light: PointLight {
                        intensity: 1000.0,
                        color: Color::ORANGE,
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                });
                *done = true;
            }
        } else {
            *done = false;
        }
    }
}

pub fn check_puzzle_both() {
    // println!("check puzzle both");
}
