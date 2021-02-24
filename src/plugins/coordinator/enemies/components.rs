use crate::coordinator::enemies::vars::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub type EnemySpriteMap<T> = HashMap<EnemySpriteKV<EnemyTypeKey<T>>, EnemySpriteKV<T>>;
pub type EnemySpriteMapKey = EnemySpriteKV<EnemyTypeKey<enemies::States>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnemySpriteKV<T> {
    State(T),
    Handle(Handle<TextureAtlas>),
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnemyTypeKey<T> {
    AngryPig(T),
    Bat(T),
    Bee(T),
    BlueBird(T),
    Bunny(T),
    Chameleon(T),
    Chicken(T),
    Duck(T),
    FatBird(T),
    Ghost(T),
    Mushroom(T),
    Plant(T),
    Radish(T),
    Rino(T),
    Rocks(T),
    Skull(T),
    Slime(T),
    Snail(T),
    Trunk(T),
    Turtle(T),
}

pub struct Enemy;

#[derive(Debug, Clone)]
pub struct Enemies {
    pub textures: EnemySpriteMap<enemies::States>,
}

impl Default for Enemies {
    fn default() -> Self {
        Self {
            textures: EnemySpriteMap::<enemies::States>::new(),
        }
    }
}
