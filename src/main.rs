mod game;
mod map;
mod player;
mod skymap;
use bevy::prelude::*;

use bevy_panorbit_camera::PanOrbitCameraPlugin;
use game::InGamePlugin;
use skymap::SkymapPlugin;

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
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(SkymapPlugin)
        .add_plugin(InGamePlugin)
        .add_plugin(player::PlayerPlugin)
        .run();
}
