use crate::puzzles::components::*;
use bevy::prelude::*;
use seldom_state::prelude::*;

pub(crate) fn get_state_machine() -> StateMachine {
    let plate_pressed = PlatePressed { range: 0.5 };
    StateMachine::default()
        .trans::<Unsolved>(plate_pressed, Solved)
        .trans::<Solved>(plate_pressed.not(), Unsolved)
        .set_trans_logging(true)
}

#[derive(Clone, Copy, FromReflect, Reflect)]
pub struct PlatePressed {
    range: f32,
}

impl BoolTrigger for PlatePressed {
    type Param<'w, 's> = (
        Query<'w, 's, &'static Transform, With<Box>>,
        Query<'w, 's, &'static Name, With<Box>>,
        Query<'w, 's, &'static Name, With<Plate>>,
        Query<'w, 's, &'static Transform, With<Plate>>,
    );
    fn trigger(
        &self,
        entity: Entity,
        (piece_pos, piece_name, plate_name, plate_pos): Self::Param<'_, '_>,
    ) -> bool {
        // If the box is on the plate, return true
        // TODO: Check state before running below code
        if let Ok(piece_pos) = piece_pos.get_single() {
            if let Ok(plate_pos) = plate_pos.get(entity) {
                if piece_pos.translation.distance(plate_pos.translation) < self.range {
                    if plate_name.get_single().unwrap() == piece_name.get_single().unwrap() {
                        return true;
                    }
                }
            }
        }

        false
    }
}

pub fn unsolved(
    unsolved: Query<(Entity, &Unsolved)>,
    mut light: Query<(&mut PointLight, With<Light>)>,
) {
    for (_entity, _unsolved) in &unsolved {
        if let Ok(mut ligth) = light.get_single_mut() {
            ligth.0.intensity = 0.0;
        }
    }
}
pub fn solved(
    mut done: Local<bool>,
    mut light: Query<(&mut PointLight, With<Light>)>,
    solved: Query<(Entity, &Solved)>,
) {
    for (_entity, _solved) in &solved {
        println!("solved");
        if !*done {
            if let Ok(mut ligth) = light.get_single_mut() {
                ligth.0.intensity = 1000.0;
            }
            *done = true;
        } else {
            *done = false;
        }
    }
}
