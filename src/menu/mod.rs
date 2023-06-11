use bevy::prelude::*;

use crate::components::AppState;

use self::systems::{
    button_system, despawn_screen, main_menu_setup, menu_action,
    menu_setup, MenuState, OnMainMenuScreen, setup,
};
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::Menu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_state::<MenuState>()
            .add_system(menu_setup.in_schedule(OnEnter(AppState::InMenu)))
            // Systems to handle the main menu scree
            .add_systems((
                main_menu_setup.in_schedule(OnEnter(MenuState::Main)),
                despawn_screen::<OnMainMenuScreen>.in_schedule(OnExit(MenuState::Main)),
            ))
            // Common systems to all screens that handles buttons behaviour
            .add_systems((menu_action, button_system).in_set(OnUpdate(AppState::InMenu)));
    }
}
