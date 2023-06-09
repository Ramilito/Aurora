use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    AssetLoading,
    AssetLoaded,
    InGame,
    InMenu
}
