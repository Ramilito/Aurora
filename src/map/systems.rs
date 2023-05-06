use bevy::prelude::*;

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
