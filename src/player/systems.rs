use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::loading::MyAssets;

use super::components::Player;

pub const PLAYER_SPEED: f32 = 2.0;

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
    for (collider, transform) in _my_assets.player_colliders.iter() {
        let mut transform = Transform::from(transform.clone());
        transform.translation = Vec3::new(0.0, 3.0, 18.0);

        commands
            .spawn(PlayerBundle::new(_my_assets.player.clone()))
            .insert(RigidBody::KinematicPositionBased)
            .insert(collider.clone())
            .insert(KinematicCharacterController {
                offset: CharacterLength::Absolute(0.1),
                ..default()
            })
            .insert(TransformBundle::from_transform(transform.clone()));
    }
}

// pub fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
//     for (entity, output) in controllers.iter() {
//         println!(
//             "Entity {:?} moved by {:?} and touches the ground: {:?}",
//             entity, output.effective_translation, output.grounded
//         );
//     }
// }

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    for mut controller in controllers.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, -1.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, -1.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(-1.0, -1.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(1.0, -1.0, 1.0);
        }

        if !keyboard_input.any_pressed([KeyCode::Right, KeyCode::Left, KeyCode::Up, KeyCode::Down])
        {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        controller.translation = Some(direction * PLAYER_SPEED * time.delta_seconds());
        controller.snap_to_ground = Some(CharacterLength::Absolute(0.5));
    }
}
