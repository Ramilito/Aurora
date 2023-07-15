use super::components::*;
use super::state_machine;
use crate::game::loading::MyAssets;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut _my_assets: ResMut<MyAssets>,
    mut commands: Commands,
) {
}

pub fn load_assets(_my_assets: Res<MyAssets>) {}

pub fn load_puzzle(_my_assets: Res<MyAssets>, mut commands: Commands) {}

pub fn load_map(_my_assets: Res<MyAssets>, mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-5.0, -5.0, 0.0),
        point_light: PointLight {
            intensity: 10000.0,
            range: 1000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands
        .spawn(SceneBundle {
            scene: _my_assets.crystal.clone(),
            transform: Transform::from_xyz(-0.0, -10.0, 0.0),
            ..default()
        })
        .with_children(|children| {
            for (collider, transform) in _my_assets.crystal_colliders.iter() {
                children.spawn((
                    RigidBody::Fixed,
                    collider.clone(),
                    TransformBundle::from(transform.clone()),
                ));
            }
        });
}
