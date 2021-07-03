use bevy::reflect::TypeUuid;
use std::collections::HashMap;

use crate::plugins::{
    animation::components::{EntSpriteKV, EntTypeKey},
    coordinator::{AnimationType as EnemyAnimationType, EnemyType},
    player::components::{AnimationType as PlayerAnimationType, PlayerType},
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
    pub player: SpriteMap<PlayerAnimationType, PlayerType>,
    pub enemies: SpriteMap<EnemyAnimationType, EnemyType>,
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
            player: SpriteMap::new(),
            enemies: SpriteMap::new(),
        }
    }
}
