use super::{components::*, state_machine};
use crate::game::loading::MyAssets;
use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;
use seldom_state::prelude::StateMachine;

#[derive(Bundle)]
pub struct PlateBundle {
    pub plate: Plate,
    name: Name,
    #[bundle]
    scene_bundle: SceneBundle,
    default_state: Unsolved,
    state_machine: StateMachine,
}

impl PlateBundle {
    pub fn new(scene: Handle<Scene>, transform: Transform, name: &str) -> Self {
        return PlateBundle {
            name: Name::new(name.to_string()),
            state_machine: state_machine::get_state_machine(),
            default_state: Unsolved,
            plate: Plate,
            scene_bundle: SceneBundle {
                scene,
                transform: transform.with_scale(Vec3::new(1.5, 2.0, 1.5)),
                ..default()
            },
        };
    }
}

pub fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut _my_assets: ResMut<MyAssets>,
    mut commands: Commands,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands
        .spawn((
            Box,
            Name::new("left"),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.6 })),
                material: debug_material.clone(),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.3, 0.3, 0.3))
        .insert(RigidBody::Dynamic)
        // .insert(TransformBundle::from(Transform::from_xyz(0.0, 3.0, 14.0)))
        .insert(TransformBundle::from(Transform::from_xyz(-3.0, 3.0, 3.0)))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(1.0));
    commands
        .spawn((
            Box,
            Name::new("right"),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.6 })),
                material: debug_material.clone(),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.3, 0.3, 0.3))
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 3.0, 3.0)))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(1.0));
}

pub fn load_assets(_my_assets: Res<MyAssets>) {}

pub fn load_puzzle(_my_assets: Res<MyAssets>, mut commands: Commands) {
    commands
        .spawn((
            Sword,
            SceneBundle {
                scene: _my_assets.sword.clone(),
                ..default()
            },
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController { ..default() })
        .insert(Collider::cuboid(0.1, 0.1, 0.1))
        .insert(TransformBundle::from(
            Transform::from_scale(Vec3::new(0.25, 0.25, 0.25))
                .with_translation(Vec3::new(0.0, 1.8, 0.5)),
        ))
        .with_children(|children| {
            for (collider, transform) in _my_assets.sword_colliders.iter() {
                children.spawn((collider.clone(), TransformBundle::from(transform.clone())));
            }
        });

    commands
        .spawn(PlateBundle::new(
            _my_assets.platform.clone(),
            Transform::from_xyz(-2.0, 0.590, 3.0),
            "left",
        ))
        .with_children(|children| {
            for (collider, transform) in _my_assets.platform_colliders.iter() {
                children.spawn((
                    RigidBody::Fixed,
                    collider.clone(),
                    TransformBundle::from(transform.clone()),
                ));
            }
            children.spawn((
                Light,
                PointLightBundle {
                    transform: Transform::from_xyz(0.0, 0.1, 0.0),
                    point_light: PointLight {
                        intensity: 0.0,
                        color: Color::ORANGE,
                        shadows_enabled: false,
                        ..default()
                    },
                    ..default()
                },
            ));
        });

    commands
        .spawn(PlateBundle::new(
            _my_assets.platform.clone(),
            Transform::from_xyz(2.0, 0.590, 3.0),
            "right",
        ))
        .with_children(|children| {
            for (collider, transform) in _my_assets.platform_colliders.iter() {
                children.spawn((
                    RigidBody::Fixed,
                    collider.clone(),
                    TransformBundle::from(transform.clone()),
                ));
            }
            children.spawn((
                Light,
                PointLightBundle {
                    transform: Transform::from_xyz(0.0, 0.1, 0.0),
                    point_light: PointLight {
                        intensity: 0.0,
                        color: Color::ORANGE,
                        shadows_enabled: false,
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

pub fn load_map(
    _my_assets: Res<MyAssets>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    asset_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    if let Some(tower) = asset_gltf.get(&_my_assets.tower_gltf) {
        let commons = TransformBundle::from(
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5)).with_rotation(
                Quat::from_euler(
                    EulerRot::XYZ,
                    (-90.0_f32).to_radians(),
                    (0.0_f32).to_radians(),
                    (0.0_f32).to_radians(),
                ),
            ),
        );
        commands
            .spawn(SceneBundle {
                scene: tower.scenes[0].clone(),
                ..default()
            })
            .insert(commons);
        commands
            .spawn(RigidBody::Fixed)
            .insert(Restitution::coefficient(0.0))
            .insert(Dominance::group(1))
            .insert(commons)
            .with_children(|children| {
                for mesh_handle in tower.named_meshes.iter() {
                    let (name, gltf_mesh) = mesh_handle;
                    if !name.contains("collider") {
                        continue;
                    }

                    let mesh = assets_gltfmesh.get(gltf_mesh).clone().unwrap();
                    let collider = Collider::from_bevy_mesh(
                        meshes.get(&mesh.primitives[0].mesh.clone()).unwrap(),
                        &ComputedColliderShape::TriMesh,
                    )
                    .unwrap();

                    children
                        .spawn(collider)
                        .insert(Dominance::group(1))
                        .insert(Restitution::coefficient(0.0))
                        .insert(TransformBundle::from(Transform::default()));
                }
                children.spawn(RigidBody::Fixed);
            });
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
