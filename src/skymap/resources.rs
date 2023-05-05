use bevy::prelude::{Resource, Handle, Image};

#[derive(Resource)]
pub struct Cubemap {
    pub is_loaded: bool,
    pub index: usize,
    pub image_handle: Handle<Image>,
}

