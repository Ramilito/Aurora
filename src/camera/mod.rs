use bevy::prelude::*;
// use bevy_panorbit_camera::PanOrbitCameraPlugin;
pub mod compontents;
mod systems;

use self::systems::{setup, sync_player_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugin(PanOrbitCameraPlugin)
        app.add_systems(Startup, setup).add_systems(Update, sync_player_camera);
    }
}
