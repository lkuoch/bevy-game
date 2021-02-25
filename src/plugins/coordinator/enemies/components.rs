use crate::common::components::*;
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
}

pub struct Enemy;

#[derive(Debug, Clone)]
pub struct Enemies {
    pub textures: EnemySpriteMap,
}

impl Default for Enemies {
    fn default() -> Self {
        Self {
            textures: EnemySpriteMap::new(),
        }
    }
}
