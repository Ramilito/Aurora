use bevy::prelude::*;
mod compontents;
mod resources;
mod systems;
use systems::setup;

use self::{compontents::CubemapMaterial, systems::asset_loaded};
pub struct SkymapPlugin;

impl Plugin for SkymapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<CubemapMaterial>::default())
            .add_startup_system(setup)
            .add_system(asset_loaded);
    }
}
