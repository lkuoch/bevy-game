use bevy::prelude::*;

pub trait Animatable<T, R = ()> {
    fn get_texture_handle_from_state(&self, handle: Handle<TextureAtlas>, resource: R)
        -> Option<T>;

    fn get_state_from_texture_handle(&self, state: T, resource: R) -> Option<Handle<TextureAtlas>>;
}
