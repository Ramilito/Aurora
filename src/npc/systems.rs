use crate::components::MyAssets;

use super::components::{Animations, Npc};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

#[derive(Bundle)]
pub struct NpcBundle {
    pub npc: Npc,
    #[bundle]
    pub scene_bundle: SceneBundle,
}

impl NpcBundle {
    pub fn new(scene: Handle<Scene>) -> Self {
        NpcBundle {
            npc: Npc,
            scene_bundle: SceneBundle { scene, ..default() },
        }
    }
}

pub fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("models/animated/npc_emo.gltf#Animation2"),
        asset_server.load("models/animated/npc_emo.gltf#Animation1"),
        asset_server.load("models/animated/npc_emo.gltf#Animation0"),
    ]));
}

pub fn load_assets(_my_assets: Res<MyAssets>, mut commands: Commands) {
    commands
        .spawn(NpcBundle::new(_my_assets.npc_emo.clone()))
        .insert(Collider::cuboid(0.4, 1.0, 0.8))
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(
            Transform::from_xyz(0.0, 1.0, 16.0).with_scale(Vec3::new(0.5, 0.5, 0.5)),
        ));
}

pub fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[1].clone_weak()).repeat();
            *done = true;
        }
    }
}
