use bevy::prelude::*;

use crate::plugins::resource_manager::components::ResourceManager;

pub trait Animatable<T> {
    fn get_texture_handle_from_state(
        &self,
        handle: Handle<TextureAtlas>,
        resource_manager: &ResourceManager,
    ) -> Option<T>;

    fn get_state_from_texture_handle(
        &self,
        state: T,
        resource_manager: &ResourceManager,
    ) -> Option<Handle<TextureAtlas>>;
}
