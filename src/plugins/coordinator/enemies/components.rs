use crate::plugins::core::{components::*, traits::*};
use bevy::prelude::*;
use std::collections::HashMap;

pub type EnemySpriteMap = HashMap<EnemySpriteMapKey, EntSpriteKV<States>>;
pub type EnemySpriteMapKey = EntSpriteKV<EntTypeKey<States, EnemyType>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnemyType {
    AngryPig,
    Bat,
    Bee,
    BlueBird,
    Bunny,
    Chameleon,
    Chicken,
    Duck,
    FatBird,
    Ghost,
    Mushroom,
    Plant,
    Radish,
    Rino,
    Rocks,
    Skull,
    Slime,
    Snail,
    Trunk,
    Turtle,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum States {
    Idle,
    DoubleJump,
    Fall,
    Hit,
    Hit2,
    Jump,
    Run,
    WallJump,
    Walk,
    CeilingIn,
    CeilingOut,
    Flying,
    Attack,
    Bullet,
    BulletPieces,
}

#[derive(Debug, Copy, Clone)]
pub struct EnemyTag {
    pub current_sprite: EnemyType,
    pub previous_sprite: EnemyType,
}

#[derive(Debug, Clone)]
pub struct Enemies {
    pub textures: EnemySpriteMap,
}

impl Animatable<States, Enemies> for EnemyTag {
    fn get_texture_handle_from_state(
        &self,
        handle: Handle<TextureAtlas>,
        resource: Enemies,
    ) -> Option<States> {
        if let Some(x) = resource.textures.get(&EntSpriteKV::Handle(handle)) {
            match x {
                EntSpriteKV::State(s) => Some(*s),
                _ => None,
            }
        } else {
            None
        }
    }

    fn get_state_from_texture_handle(
        &self,
        state: States,
        resource: Enemies,
    ) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = resource.textures.get(&EntSpriteKV::State(EntTypeKey {
            ty: self.current_sprite,
            state,
        })) {
            match x {
                EntSpriteKV::Handle(h) => Some(h.clone()),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Default for Enemies {
    fn default() -> Self {
        Self {
            textures: EnemySpriteMap::new(),
        }
    }
}
