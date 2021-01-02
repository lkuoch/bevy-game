use crate::player::vars::mask_dude as MaskDude;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct Player {
    pub textures: HashMap<MaskDude::States, Handle<TextureAtlas>>,
}

impl Player {
    pub fn get_animation(&self, animation: MaskDude::States) -> Option<Handle<TextureAtlas>> {
        self.textures.get(&animation).cloned()
    }
}
