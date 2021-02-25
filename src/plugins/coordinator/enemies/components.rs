use crate::coordinator::enemies::vars::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub type EnemySpriteMap<T> = HashMap<EnemySpriteKV<EnemyTypeKey<T>>, EnemySpriteKV<T>>;
pub type EnemySpriteMapKey = EnemySpriteKV<EnemyTypeKey<States>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnemySpriteKV<T> {
    State(T),
    Handle(Handle<TextureAtlas>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EnemyTypeKey<T> {
    pub state: T,
    pub ty: EnemyType,
}

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
    pub textures: EnemySpriteMap<States>,
}

impl Default for Enemies {
    fn default() -> Self {
        Self {
            textures: EnemySpriteMap::<States>::new(),
        }
    }
}
