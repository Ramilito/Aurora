use crate::puzzles::components::*;
use bevy::prelude::*;
use seldom_state::prelude::*;

pub(crate) fn get_state_machine(name: String) -> StateMachine {
    let plate_pressed = PlatePressed { range: 0.5 };
    StateMachine::default()
        .trans::<Unsolved>(plate_pressed, Solved)
        .trans::<Solved>(plate_pressed.not(), Unsolved)
    // .set_trans_logging(true)
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
        // If the box is on the plate, return true
        // TODO: Check state before running below code

        if let Ok(plate) = plates.get(entity) {
            for piece in pieces.iter() {
                if piece.0.translation.distance(plate.0.translation) < self.range {
                    if piece.1 == plate.1 {
                        return true;
                    }
                }
            }
        }
        // for piece in pieces.iter() {
        //     for plate in plates.iter() {
        //         if piece.0.translation.distance(plate.0.translation) < self.range {
        //             if piece.1 == plate.1 {
        //                 return true;
        //             }
        //         }
        //     }
        // }

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
    solved: Query<(Entity, &Solved)>,
    q_parent: Query<(&Plate, &Name, &Children)>,
    mut q_child: Query<(&mut PointLight, With<Light>)>,
) {
    for (entity, _solved) in &solved {
        for (plate, name, children) in q_parent.get(entity).iter() {
            println!("test: {:?}", name);

            for &child in children.iter() {
                if let Ok(mut ligth) = q_child.get_mut(child) {
                    ligth.0.intensity = 1000.0;
                }
            }
        }
    }
}
