use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct PuzzleBoxLeft;

#[derive(Component)]
pub struct PuzzlePlateLeft;

#[derive(Component)]
pub struct PuzzlePlateRight;

#[derive(Component)]
pub struct PuzzleBoxRight;

#[derive(Bundle)]
pub struct PuzzleBundle {
    pub puzzle_box_left: PuzzleBoxLeft,
    pub puzzle_box_rigth: PuzzleBoxRight,
}

pub fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands
        .spawn((
            PuzzleBoxLeft,
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::ALICE_BLUE.into()),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.3, 0.3, 0.3))
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 3.0, 14.0)))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0));

    commands.spawn((
        PuzzlePlateLeft,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(1.0))),
            material: materials.add(Color::ALICE_BLUE.into()),
            transform: Transform::from_xyz(2.0, 0.630, 3.0),
            ..default()
        },
    ));

    commands.spawn((
        PuzzlePlateRight,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(1.0))),
            material: materials.add(Color::DARK_GREEN.into()),
            transform: Transform::from_xyz(-2.0, 0.630, 3.0),
            ..default()
        },
    ));
    commands
        .spawn((
            PuzzleBoxRight,
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::DARK_GREEN.into()),
                // transform: Transform::from_xyz(6.0, 3.5, 0.0),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.3, 0.3, 0.3))
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 3.0, 14.0)))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0));
}

pub fn check_puzzle_right(
    query_plate: Query<&Transform, With<PuzzlePlateRight>>,
    query_box: Query<&Transform, With<PuzzleBoxRight>>,
) {
    let _plate = query_plate.single();
    let _box = query_box.single();

    let distance = _plate.translation.distance(_box.translation);
    if distance < 1.0 {
        println!("check puzzle right");
    }
}

pub fn check_puzzle_left(
    query_plate: Query<&Transform, With<PuzzlePlateLeft>>,
    query_box: Query<&Transform, With<PuzzleBoxLeft>>,
) {
    let _plate = query_plate.single();
    let _box = query_box.single();

    let distance = _plate.translation.distance(_box.translation);
    if distance < 1.0 {
        println!("check puzzle left");
    }
}

pub fn check_puzzle_both() {
    // println!("check puzzle both");
}
