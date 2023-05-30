use crate::puzzles::components::*;
use bevy::prelude::*;
use seldom_state::prelude::*;

pub(crate) fn get_state_machine() -> StateMachine {
    let plate_pressed = PlatePressed { range: 0.5 };
    StateMachine::default()
        .trans::<Unsolved>(plate_pressed, Solved)
        .trans::<Solved>(plate_pressed.not(), Unsolved)
}

#[derive(Clone, Copy, FromReflect, Reflect)]
pub struct PlatePressed {
    range: f32,
}

impl BoolTrigger for PlatePressed {
    type Param<'w, 's> = (
        Query<'w, 's, (&'static Transform, &'static Name), With<Box>>,
        Query<'w, 's, (&'static Transform, &'static Name), With<Plate>>,
    );
    fn trigger(&self, entity: Entity, (pieces, plates): Self::Param<'_, '_>) -> bool {
        // TODO: Check current state before running below code
        if let Ok(plate) = plates.get(entity) {
            for piece in pieces.iter() {
                if piece.0.translation.distance(plate.0.translation) < self.range {
                    if piece.1 == plate.1 {
                        return true;
                    }
                }
            }
        }

        false
    }
}

pub fn unsolved(
    mut done: Local<bool>,
    unsolved: Query<(Entity, &Unsolved)>,
    q_parent: Query<(&Plate, &Name, &Children)>,
    mut q_child: Query<(&mut PointLight, With<Light>)>,
) {
    if !*done {
        for (entity, _unsolved) in &unsolved {
            for (_plate, _name, children) in q_parent.get(entity).iter() {
                for &child in children.iter() {
                    if let Ok(mut ligth) = q_child.get_mut(child) {
                        ligth.0.intensity = 0.0;
                    }
                }
            }
        }
        *done = true;
    } else {
        *done = false;
    }
}

pub fn solved(
    mut done: Local<bool>,
    mut q_child: Query<(&mut PointLight, With<Light>)>,
    solved: Query<(Entity, &Solved)>,
    q_parent: Query<(&Plate, &Name, &Children)>,
) {
    if !*done {
        for (entity, _solved) in &solved {
            for (_plate, _name, children) in q_parent.get(entity).iter() {
                for &child in children.iter() {
                    if let Ok(mut ligth) = q_child.get_mut(child) {
                        ligth.0.intensity = 100.0;
                    }
                }
            }
        }

        *done = true;
    } else {
        *done = false;
    }
}
