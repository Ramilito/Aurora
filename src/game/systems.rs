use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use std::f32::consts::{PI, TAU};

use crate::player::components::Player;

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(300.0).into()),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
    commands.spawn(SceneBundle {
        scene: asset_server.load("map/tower.glb#Scene0"),
        ..default()
    });

    commands.spawn((
        Camera3dBundle::default(),
        PanOrbitCamera {
            // Set focal point
            focus: Vec3::new(0.0, 1.0, 0.0),
            // Set the starting position
            alpha: TAU / 8.0,
            beta: TAU / 8.0,
            radius: 10.0,
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
            button_orbit: MouseButton::Middle,
            button_pan: MouseButton::Middle,
            modifier_pan: Some(KeyCode::LShift),
            // Reverse the zoom direction
            reversed_zoom: true,
            // Makes sure it's enabled (this is default)
            enabled: true,
            ..default()
        },
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });
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
