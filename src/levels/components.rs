use bevy::prelude::*;

#[derive(Component, Clone, Reflect)]
#[component(storage = "SparseSet")]
pub struct Unsolved;

#[derive(Component, Clone, Reflect, Debug)]
#[component(storage = "SparseSet")]
pub struct Solved;

#[derive(Component)]
pub struct Box;

#[derive(Component, Debug)]
pub struct Plate;

#[derive(Component, Debug)]
pub struct Light;

#[derive(Component, Debug)]
pub struct Sword;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum LevelState {
    CurrentLevel,
    LevelTwo,
    #[default]
    LevelOne,
}
