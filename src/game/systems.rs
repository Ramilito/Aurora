use bevy::{prelude::*, render::camera::ScalingMode};

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(300.0).into()),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });

    // commands.spawn(Camera3dBundle {
    //     projection: OrthographicProjection {
    //         scale: 15.0,
    //         scaling_mode: ScalingMode::FixedVertical(2.0),
    //         ..default()
    //     }
    //     .into(),
    //     transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(0.0, 15.0, 0.0),
    //     point_light: PointLight {
    //         intensity: 9000.0,
    //         color: Color::WHITE,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     ..default()
    // });
}
