use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::KayakWidgets};

use crate::components::AppState;

use self::systems::{startup, PreloadResource};
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreloadResource>()
            // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_system(startup.in_schedule(OnEnter(AppState::InMenu)));
    }
}
