use bevy::reflect::TypeUuid;
use std::collections::HashMap;

use crate::plugins::{
    animation::components::{EntSpriteKV, EntTypeKey},
    coordinator::{EnemyAnimationStates, EnemyTypes},
    player::components::{PlayerAnimationStates, PlayerTypeStates},
};

pub type SpriteMapKey<T1, T2> = EntSpriteKV<EntTypeKey<T1, T2>>;
pub type SpriteMap<T1, T2> = HashMap<SpriteMapKey<T1, T2>, EntSpriteKV<T1>>;

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "08f3050f-4e2f-4f86-9686-94ad8c20873f"]
pub struct ResourceManager {
    pub textures: Textures,
}

#[derive(Debug, Clone)]
pub struct Textures {
    pub players: SpriteMap<PlayerAnimationStates, PlayerTypeStates>,
    pub enemies: SpriteMap<EnemyAnimationStates, EnemyTypes>,
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self {
            textures: Textures::default(),
        }
    }
}

impl Default for Textures {
    fn default() -> Self {
        Self {
            players: SpriteMap::new(),
            enemies: SpriteMap::new(),
        }
    }
}
