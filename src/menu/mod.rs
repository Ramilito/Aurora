use bevy::prelude::*;

use crate::components::AppState;

use self::systems::{
    button_system, despawn_screen, main_menu_setup, menu_action, menu_setup, setup, MenuState,
    OnMainMenuScreen,
};
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::Menu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_state::<MenuState>()
            .add_systems(OnEnter(AppState::InMenu), menu_setup)
            // Systems to handle the main menu scree
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            // Common systems to all screens that handles buttons behaviour
            .add_systems(Update, (menu_action, button_system).run_if(in_state(MenuState::Main)));
    }
}
