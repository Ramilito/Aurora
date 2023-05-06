use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod compontents;
mod systems;

use self::systems::setup;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup);
    }
}
