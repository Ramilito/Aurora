use bevy::prelude::*;
mod compontents;
mod resources;
mod systems;
use systems::setup;

use self::{compontents::CubemapMaterial, systems::asset_loaded};
pub struct SkymapPlugin;

impl Plugin for SkymapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CubemapMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, asset_loaded);
    }
}
