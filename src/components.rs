use bevy::{prelude::*, ecs::schedule::ScheduleLabel};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States, ScheduleLabel)]
pub enum AppState {
    #[default]
    AssetLoading,
    AssetLoaded,
    InGame,
    InMenu
}
