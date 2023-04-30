mod game;
mod helpers;
mod map;
mod player;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use game::InGamePlugin;
use map::MapPlugin;
use player::PlayerPlugin;

#[derive(Resource)]
struct BonusSpawnTimer(Timer);

fn main() {
    App::new()
        .insert_resource(TilemapRenderSettings {
            // Map size is 12x12 so we'll have render chunks that are:
            // 12 tiles wide and 1 tile tall.
            render_chunk_size: UVec2::new(3, 1),
        })
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
        .add_plugin(TilemapPlugin)
        .add_plugin(helpers::tiled::TiledMapPlugin)
        .add_plugin(MapPlugin)
        // .add_plugin(InGamePlugin)
        // .add_plugin(PlayerPlugin)
        .add_system(helpers::camera::movement)
        .run();
}
