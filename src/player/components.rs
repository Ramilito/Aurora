use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Jumper {
    pub is_jumping: bool,
}
