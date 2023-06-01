use crate::puzzles::components::*;
use bevy::prelude::*;
use seldom_state::prelude::*;

pub(crate) fn get_state_machine() -> StateMachine {
    let plate_pressed = PlatePressed { range: 0.8 };
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
    mut child_q: Query<(&mut PointLight, With<Light>)>,
    solved: Query<(Entity, &Solved)>,
    parent_q: Query<(&Plate, &Name, &Children)>,
    mut sword_q: Query<&mut Transform, With<Sword>>,
) {
    if !*done {
        for (entity, _solved) in &solved {
            for (_plate, name, children) in parent_q.get(entity).iter() {
                if name.contains("left") {
                    if let Ok(mut transform) = sword_q.get_single_mut() {
                        transform.translation = Vec3::new(0.0, 1.5, 3.0);
                    }
                }
                for &child in children.iter() {
                    if let Ok(mut light) = child_q.get_mut(child) {
                        light.0.intensity = 100.0;
                    }
                }
            }
        }
        *done = true;
    } else {
        *done = false;
    }
}
