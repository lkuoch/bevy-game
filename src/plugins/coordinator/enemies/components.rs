use crate::plugins::core::{components::*, traits::*};
use bevy::prelude::*;
use std::collections::HashMap;

pub type EnemySpriteMap = HashMap<EnemySpriteMapKey, EntSpriteKV<AnimationType>>;
pub type EnemySpriteMapKey = EntSpriteKV<EntTypeKey<AnimationType, EnemyType>>;

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
    Generic,
    Ghost,
    Mushroom,
    Plant,
    Radish,
    Rhino,
    Rocks1,
    Rocks2,
    Rocks3,
    Skull,
    Slime,
    Snail,
    Trunk,
    Turtle,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum AnimationType {
    Appear,
    Attack,
    Bullet,
    BulletPieces,
    CeilingIn,
    CeilingOut,
    Disappear,
    Fall,
    Flying,
    GhostParticles,
    Ground,
    Hit,
    Hit2,
    HitWall,
    HitWall2,
    Idle,
    Idle2,
    IdleRun,
    Jump,
    JumpAnticipation,
    Leafs,
    RedParticle,
    ShellIdle,
    ShellTopHit,
    ShellWallHit,
    SlimeParticles,
    SnailWithoutShell,
    TurtleSpikesIn,
    TurtleSpikesOut,
    OrangeParticle,
    Run,
    Walk,
}

#[derive(Debug, Copy, Clone)]
pub struct EnemyTag {
    pub current_sprite: EnemyType,
}

#[derive(Debug, Clone)]
pub struct Enemies {
    pub textures: EnemySpriteMap,
}

impl Animatable<AnimationType, Enemies> for EnemyTag {
    fn get_texture_handle_from_state(
        &self,
        handle: Handle<TextureAtlas>,
        resource: Enemies,
    ) -> Option<AnimationType> {
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
        state: AnimationType,
        resource: Enemies,
    ) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = resource.textures.get(&EntSpriteKV::State(EntTypeKey {
            ty: self.current_sprite,
            anim_ty: state,
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
