use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use seldom_state::prelude::*;

use crate::player::components::Player;

use super::components::{Animations, NpcEmo};
pub(crate) fn get_state_machine() -> StateMachine {
    let near_player = NearPlayer { range: 2.0 };
    StateMachine::default()
        .trans::<Idle>(near_player, InDialog)
        .trans::<InDialog>(near_player.not(), Idle)
}

#[derive(Component, Clone, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;

#[derive(Component, Clone, Reflect)]
#[component(storage = "SparseSet")]
pub struct InDialog;

#[derive(Clone, Copy, FromReflect, Reflect)]
pub struct NearPlayer {
    range: f32,
}

impl BoolTrigger for NearPlayer {
    type Param<'w, 's> = (
        Query<'w, 's, &'static Transform, With<NpcEmo>>,
        Query<'w, 's, &'static Transform, With<Player>>,
    );
    fn trigger(&self, entity: Entity, (enemies, player): Self::Param<'_, '_>) -> bool {
        if let Ok(enemy_transform) = enemies.get(entity) {
            if let Ok(player_transform) = player.get_single() {
                let distance = enemy_transform.translation.distance(player_transform.translation);
                if distance < 2.0 {
                    // next_state.set(AppState::InDialog);
                    return true;
                }
            }
        }

        false
    }
}

pub fn idle(
    animations: Res<Animations>,
    mut animation_player: Query<&mut AnimationPlayer>,
    idle: Query<(Entity, &Idle)>,
) {
    for (_entity, _idle) in &idle {
        if let Ok(mut player) = animation_player.get_single_mut() {
            player
                .play_with_transition(animations.0[11].clone_weak(), Duration::from_millis(250))
                .repeat();
        }
    }
}

pub fn indialog(
    animations: Res<Animations>,
    mut animation_player: Query<&mut AnimationPlayer>,
    indialog: Query<(Entity, &InDialog)>,
    windows: Query<&Window>,
    mut contexts: EguiContexts,
) {
    let window = windows.single();

    for (_entity, _indialog) in &indialog {
        egui::Window::new("Hello")
            .fixed_pos(egui::pos2(window.width() / 2.0, window.height() / 1.4))
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Aurora...");
            });
        if let Ok(mut player) = animation_player.get_single_mut() {
            player
                .play_with_transition(animations.0[6].clone_weak(), Duration::from_millis(250))
                .repeat();
        }
    }
}
