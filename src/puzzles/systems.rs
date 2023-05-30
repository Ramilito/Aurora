use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use seldom_state::prelude::StateMachine;

use crate::game::loading::MyAssets;

// use super::state_machine::{self, Unsolved};
use super::{components::*, state_machine};

#[derive(Bundle)]
pub struct PlateBundle {
    pub plate: Plate,
    name: Name,
    #[bundle]
    scene_bundle: SceneBundle,
    default_state: Unsolved,
    state_machine: StateMachine,
}

impl PlateBundle {
    pub fn new(scene: Handle<Scene>, transform: Transform, name: &str) -> Self {
        return PlateBundle {
            name: Name::new(name.to_string()),
            state_machine: state_machine::get_state_machine(name.to_string()),
            default_state: Unsolved,
            plate: Plate,
            scene_bundle: SceneBundle {
                scene,
                transform: transform.with_scale(Vec3::new(1.5, 2.0, 1.5)),
                ..default()
            },
        };
    }
}

pub fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands
        .spawn((
            Box,
            Name::new("left"),
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
        .insert(ColliderMassProperties::Density(1.0));
    commands
        .spawn((
            Box,
            Name::new("right"),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::ALICE_BLUE.into()),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.3, 0.3, 0.3))
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 3.0, 3.0)))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(1.0));
}

pub fn load_assets(_my_assets: Res<MyAssets>, mut commands: Commands) {
    commands
        .spawn(PlateBundle::new(
            _my_assets.platform.clone(),
            Transform::from_xyz(-2.0, 0.515, 3.0),
            "left",
        ))
        .with_children(|children| {
            for (collider, transform) in _my_assets.platform_colliders.iter() {
                children.spawn((
                    RigidBody::Fixed,
                    collider.clone(),
                    TransformBundle::from(transform.clone()),
                ));
            }
            children.spawn((
                Light,
                PointLightBundle {
                    transform: Transform::from_xyz(0.0, 0.715, 0.0),
                    point_light: PointLight {
                        intensity: 0.0,
                        color: Color::ORANGE,
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                },
            ));
        });

    commands
        .spawn(PlateBundle::new(
            _my_assets.platform.clone(),
            Transform::from_xyz(2.0, 0.515, 3.0),
            "right",
        ))
        .with_children(|children| {
            for (collider, transform) in _my_assets.platform_colliders.iter() {
                children.spawn((
                    RigidBody::Fixed,
                    collider.clone(),
                    TransformBundle::from(transform.clone()),
                ));
            }
            children.spawn((
                Light,
                PointLightBundle {
                    transform: Transform::from_xyz(0.0, 0.715, 0.0),
                    point_light: PointLight {
                        intensity: 0.0,
                        color: Color::ORANGE,
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}
