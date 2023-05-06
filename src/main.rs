mod camera;
mod collision;
mod game;
mod map;
mod player;
mod skymap;

use bevy::prelude::*;

#[derive(Resource)]
struct BonusSpawnTimer(Timer);

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
        .add_plugin(camera::CameraPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(game::InGamePlugin)
        .run();
}
