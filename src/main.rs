mod camera;
mod collision;
mod game;
mod map;
mod player;
mod skymap;

use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_rapier3d::prelude::{
    Collider, ColliderMassProperties, ComputedColliderShape, Dominance, GravityScale, Restitution,
    RigidBody, Velocity,
};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AppState {
    #[default]
    AssetLoading,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Iso Staggered Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::InGame),
        )
        .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading)
        .add_system(use_my_assets.in_schedule(OnEnter(AppState::InGame)))
        .add_plugin(camera::CameraPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(game::InGamePlugin)
        .run();
}
fn use_my_assets(
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    _my_assets: Res<MyAssets>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
) {
    if let Some(tower) = meshes.get(&_my_assets.tower_mesh) {
        let m = Collider::from_bevy_mesh(tower, &ComputedColliderShape::TriMesh).unwrap();
        commands
            .spawn(SceneBundle {
                scene: _my_assets.tower.clone(),
                ..default()
            })
            .insert(m)
            .insert(RigidBody::Fixed)
            .insert(Restitution::coefficient(0.0))
            .insert(Dominance::group(0))
            .insert(TransformBundle::from(
                Transform::from_xyz(0.0, -3.0, 0.0)
                    .with_scale(Vec3::new(0.5, 0.5, 0.5))
                    .with_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        (-90.0_f32).to_radians(),
                        (0.0_f32).to_radians(),
                        (0.0_f32).to_radians(),
                    )),
            ));
    } else {
        info!("sphere hasn't loaded yet");
    }
}
#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(path = "map/tower.gltf#Scene0")]
    tower: Handle<Scene>,

    #[asset(path = "map/tower.gltf#Mesh0/Primitive0")]
    tower_mesh: Handle<Mesh>,

    #[asset(path = "models/AlienCake/alien.glb#Scene0")]
    player: Handle<Scene>,
}
