use bevy::prelude::*;
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

pub fn load_assets(_my_assets: Res<MyAssets>, mut commands: Commands) {
    commands
        .spawn((
            PuzzlePlateRight,
            SceneBundle {
                scene: _my_assets.platform.clone(),
                ..default()
            },
        ))
        .insert(TransformBundle::from(
            Transform::from_xyz(2.0, 0.615, 3.0).with_scale(Vec3::new(1.5, 0.5, 1.5)),
        ));

    for (collider, transform) in _my_assets.platform_colliders.iter() {
        println!("collider: {:?}", transform);
        commands
            .spawn(RigidBody::Fixed)
            .insert(collider.clone())
            .insert(Dominance::group(1))
            .insert(Restitution::coefficient(0.0))
            .insert(TransformBundle::from(
                transform
                    .clone()
                    .with_translation(Vec3::new(2.0, 0.615, 3.0))
                    .with_scale(Vec3::new(1.5, 0.5, 1.5)),
            ));
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
