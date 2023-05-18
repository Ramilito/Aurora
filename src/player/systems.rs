use bevy_rapier3d::prelude::CharacterLength;

use crate::game::loading::MyAssets;

use super::components::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};

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
    commands
        .spawn(PlayerBundle::new(_my_assets.player.clone()))
        .insert(Collider::cuboid(0.25, 0.4, 0.2))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController { ..default() })
        // .insert(Dominance::group(10))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 18.0)));
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
