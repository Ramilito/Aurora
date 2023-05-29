use bevy::prelude::*;

#[derive(Component, Clone, Reflect)]
#[component(storage = "SparseSet")]
pub struct Unsolved;

#[derive(Component, Clone, Reflect)]
#[component(storage = "SparseSet")]
pub struct Solved;

#[derive(Component)]
pub struct PuzzleBoxLeft;

#[derive(Component)]
pub struct PuzzlePlateLeft;

#[derive(Component)]
pub struct PuzzlePlateRight;

#[derive(Component)]
pub struct PuzzleBoxRight;

#[derive(Component)]
pub struct PuzzleLightRight;

#[derive(Component)]
pub struct PuzzleLightLeft;

#[derive(Component)]
pub struct Box;

#[derive(Component)]
pub struct Plate;

#[derive(Component)]
pub struct Light;
