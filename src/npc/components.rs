use bevy::prelude::*;

#[derive(Component)]
pub struct Npc;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);
