use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
mod compontents;
mod systems;

use self::systems::{setup, sync_player_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PanOrbitCameraPlugin)
            .add_startup_system(setup)
            .add_system(sync_player_camera);
    }
}
