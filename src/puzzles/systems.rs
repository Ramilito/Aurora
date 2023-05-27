use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use seldom_state::prelude::StateMachine;

use crate::game::loading::MyAssets;

use super::state_machine::{self, Unsolved};

#[derive(Component)]
pub struct PuzzleBoxLeft;

#[derive(Component)]
pub struct PuzzlePlateLeft;

#[derive(Component)]
pub struct PuzzlePlateRight;

#[derive(Component)]
pub struct PuzzleBoxRight;

#[derive(Component)]
pub struct PuzzleLightRight;

#[derive(Component)]
pub struct PuzzleLightLeft;

#[derive(Component)]
pub struct Box;

#[derive(Component)]
pub struct Plate;

#[derive(Component)]
pub struct Light;

#[derive(Bundle)]
pub struct PuzzleBundle {
    pub piece: Box,
    pub plate: Plate,
    pub ligth: Light,
    default_state: Unsolved,
    state_machine: StateMachine,
}

impl PuzzleBundle {
    pub fn new() -> Self {
        return PuzzleBundle {
            state_machine: state_machine::get_state_machine(),
            default_state: Unsolved,
            piece: Box,
            plate: Plate,
            ligth: Light,
        };
    }
}

pub fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {

    // commands
    //     .spawn((
    //         PuzzleBoxLeft,
    //         PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
    //             material: materials.add(Color::ALICE_BLUE.into()),
    //             ..default()
    //         },
    //     ))
    //     .insert(Collider::cuboid(0.3, 0.3, 0.3))
    //     .insert(RigidBody::Dynamic)
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 3.0, 14.0)))
    //     .insert(Restitution::coefficient(0.3))
    //     .insert(ColliderMassProperties::Density(2.0));
    //
    // commands
    //     .spawn((
    //         PuzzleBoxRight,
    //         PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
    //             material: materials.add(Color::DARK_GREEN.into()),
    //             ..default()
    //         },
    //     ))
    //     .insert(Collider::cuboid(0.3, 0.3, 0.3))
    //     .insert(RigidBody::Dynamic)
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 3.0, 14.0)))
    //     .insert(Restitution::coefficient(0.3))
    //     .insert(ColliderMassProperties::Density(2.0));
}

pub fn load_assets(_my_assets: Res<MyAssets>, mut commands: Commands) {
    commands
        .spawn((
            PuzzleBundle::new(),
            SceneBundle {
                scene: _my_assets.platform.clone(),
                transform: Transform::from_xyz(2.0, 0.615, 3.0)
                    .with_scale(Vec3::new(1.5, 2.0, 1.5)),
                ..default()
            },
        ))
        .with_children(|children| {
            for (collider, transform) in _my_assets.platform_colliders.iter() {
                children
                    .spawn(RigidBody::Fixed)
                    .insert(collider.clone())
                    .insert(TransformBundle::from(transform.clone()));
            }
        });

    commands
        .spawn(SceneBundle {
            scene: _my_assets.barrel.clone(),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 3.0, 14.0)))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|parent| {
            for (collider, transform) in _my_assets.barrel_colliders.iter() {
                parent.spawn((
                    collider.clone(),
                    TransformBundle::from_transform(*transform),
                ));
            }
        });
}

pub fn check_puzzle_right(
    query_plate: Query<&Transform, With<PuzzlePlateRight>>,
    query_box: Query<&Transform, With<PuzzleBoxRight>>,
    query_light: Query<Entity, With<PuzzleLightRight>>,
    mut commands: Commands,
    mut done: Local<bool>,
) {
    let _box = query_box.single();
    if let Ok(_plate) = query_plate.get_single() {
        let distance = _plate.translation.distance(_box.translation);

        if distance < 1.0 {
            if !*done {
                println!("check puzzle right");
                commands.spawn((
                    PuzzleLightRight,
                    PointLightBundle {
                        transform: Transform::from_xyz(2.0, 0.715, 3.0),
                        point_light: PointLight {
                            intensity: 1000.0,
                            color: Color::ORANGE,
                            shadows_enabled: true,
                            ..default()
                        },
                        ..default()
                    },
                ));
                *done = true;
            }
        } else {
            for light in query_light.iter() {
                commands.entity(light).despawn();
            }

            println!("not close");
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
                commands.spawn((
                    PuzzleLightLeft,
                    PointLightBundle {
                        transform: Transform::from_xyz(2.0, 0.715, 3.0),
                        point_light: PointLight {
                            intensity: 1000.0,
                            color: Color::ORANGE,
                            shadows_enabled: true,
                            ..default()
                        },
                        ..default()
                    },
                ));
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
