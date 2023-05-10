use crate::components::MyAssets;

use super::components::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

pub const PLAYER_SPEED: f32 = 3.0;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle]
    pub scene_bundle: SceneBundle,
}

impl PlayerBundle {
    pub fn new(scene: Handle<Scene>) -> Self {
        PlayerBundle {
            player: Player,
            scene_bundle: SceneBundle { scene, ..default() },
        }
    }
}

pub fn load_assets(_my_assets: Res<MyAssets>, mut commands: Commands) {
    commands
        .spawn(PlayerBundle::new(_my_assets.player.clone()))
        .insert(Collider::cuboid(0.25, 0.4, 0.2))
        .insert(RigidBody::Dynamic)
        // .insert(Restitution::coefficient(0.0))
        // .insert(Dominance::group(0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 18.0)));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(-1.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(1.0, 0.0, 1.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
