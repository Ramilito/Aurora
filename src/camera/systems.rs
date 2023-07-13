use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_panorbit_camera::PanOrbitCamera;
use std::f32::consts::TAU;

use super::compontents::PlayerCamera;
use crate::player::components::Player;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        PlayerCamera,
        BloomSettings {
            intensity: 0.25, // the default is 0.3
            ..default()
        },
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                order: 0,
                ..default()
            },

            ..default()
        },
        UiCameraConfig { show_ui: false },
        PanOrbitCamera {
            // Set focal point
            focus: Vec3::new(0.0, 1.0, 0.0),
            // Set the starting position
            alpha: Some(TAU / 8.0),
            beta: Some(TAU / 8.0),
            radius: Some(10.0),
            // Set limits on the position
            alpha_upper_limit: Some(TAU / 4.0),
            alpha_lower_limit: Some(-TAU / 4.0),
            beta_upper_limit: Some(TAU / 3.0),
            beta_lower_limit: Some(-TAU / 3.0),
            // Adjust sensitivity of controls
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            // Allow the camera to go upside down
            allow_upside_down: true,
            // Blender-like key bindings
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Left,
            // modifier_pan: Some(KeyCode::),
            // Reverse the zoom direction
            reversed_zoom: true,
            // Makes sure it's enabled (this is default)
            enabled: true,
            ..default()
        },
    ));
}
pub fn sync_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut PanOrbitCamera, &mut Transform), Without<Player>>,
) {
    let Ok(player) = player.get_single() else { return };
    let Ok((mut camera, mut camera_transform)) = camera.get_single_mut() else { return };

    let delta = player.translation - camera.focus;

    if delta != Vec3::ZERO {
        camera.focus = player.translation;
        camera_transform.translation += delta;
    }
}
