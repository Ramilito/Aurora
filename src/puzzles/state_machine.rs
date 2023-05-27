use bevy::prelude::*;
use seldom_state::prelude::*;

use super::systems::{Light, Box, Plate};

pub(crate) fn get_state_machine() -> StateMachine {
    let plate_pressed = PlatePressed { range: 1.0 };
    StateMachine::default()
        .trans::<Unsolved>(plate_pressed, Solved)
        .trans::<Solved>(plate_pressed.not(), Unsolved)
        .set_trans_logging(true)
}

#[derive(Component, Clone, Reflect)]
#[component(storage = "SparseSet")]
pub struct Unsolved;

#[derive(Component, Clone, Reflect)]
#[component(storage = "SparseSet")]
pub struct Solved;

#[derive(Clone, Copy, FromReflect, Reflect)]
pub struct PlatePressed {
    range: f32,
}

impl BoolTrigger for PlatePressed {
    type Param<'w, 's> = (
        Query<'w, 's, &'static Transform, With<Box>>,
        Query<'w, 's, &'static Transform, With<Plate>>,
        Query<'w, 's, &'static Transform, With<Light>>,
    );
    fn trigger(
        &self,
        entity: Entity,
        (puzzlebox, puzzleplate, puzzlelight): Self::Param<'_, '_>,
    ) -> bool {
        // if let Ok(enemy_transform) = enemies.get(entity) {
        //     if let Ok(player_transform) = player.get_single() {
        //         let distance = enemy_transform.translation.distance(player_transform.translation);
        //         if distance < 1.0 {
        //             // next_state.set(AppState::InDialog);
        //             return true;
        //         }
        //     }
        // }

        // println!("triggered");
        false
    }
}

pub fn unsolved() {
    // println!("unsolved");
}
pub fn solved(solved: Query<(Entity, &Solved)>) {
    for (_entity, _solved) in &solved {
        println!("solved");
    }
}
