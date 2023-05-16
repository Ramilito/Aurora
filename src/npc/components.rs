use bevy::prelude::*;

#[derive(Component)]
pub struct NpcEmo;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);
