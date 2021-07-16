use bevy::prelude::*;

use crate::plugins::resource_manager::components::ResourceManager;

pub trait Animatable<T, U> {
    fn get_texture_handle_from_state(
        &self,
        handle: &Handle<TextureAtlas>,
        resource_manager: &ResourceManager,
    ) -> Option<U>;

    fn get_state_from_texture_handle(
        &self,
        entity_state: &T,
        animation_state: &U,
        resource_manager: &ResourceManager,
    ) -> Option<Handle<TextureAtlas>>;
}
