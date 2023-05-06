use bevy::prelude::*;

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("map/tower.glb#Scene0"),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("map/clouds_skybox.glb#Scene0"),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        point_light: PointLight {
            intensity: 1000.0,
            color: Color::ORANGE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-5.0, 15.0, 10.0),
        point_light: PointLight {
            intensity: 10000.0,
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
