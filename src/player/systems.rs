use super::components::Player;
use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 10.0;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle]
    pub scene_bundle: SceneBundle,
}

impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>) -> Self {
        PlayerBundle {
            player: Player,
            scene_bundle: SceneBundle {
                transform: Transform::from_xyz(3.0, 0.67, 0.0),
                scene: asset_server.load("models/AlienCake/alien.glb#Scene0"),
                ..default()
            },
        }
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _player_entity = commands.spawn(PlayerBundle::new(asset_server)).id();
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
