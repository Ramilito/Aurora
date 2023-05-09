use bevy::prelude::*;

mod components;
mod systems;
use bevy_asset_loader::prelude::LoadingStateAppExt;
use systems::setup;

use crate::components::AppState;
use self::components::MyAssets;
use self::systems::load_assets;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading)
            .add_system(load_assets.in_schedule(OnEnter(AppState::InGame)))
            .add_startup_system(setup);
    }
}
