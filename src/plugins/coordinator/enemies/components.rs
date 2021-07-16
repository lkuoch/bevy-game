use crate::plugins::{
    animation::{components::*, traits::*},
    resource_manager::components::ResourceManager,
};
use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnemyTypes {
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
pub enum EnemyAnimationStates {
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
    pub current_sprite: EnemyTypes,
}

#[derive(Debug, Clone)]
pub struct Enemies {}

impl Animatable<EnemyTypes, EnemyAnimationStates> for EnemyTag {
    fn get_texture_handle_from_state(
        &self,
        handle: &Handle<TextureAtlas>,
        resource_manager: &ResourceManager,
    ) -> Option<EnemyAnimationStates> {
        if let Some(x) = resource_manager
            .textures
            .enemies
            .get(&EntSpriteKV::Handle(handle.clone()))
        {
            return match x {
                EntSpriteKV::State(s) => Some(*s),
                _ => None,
            };
        }

        None
    }

    fn get_state_from_texture_handle(
        &self,
        entity_state: &EnemyTypes,
        animation_state: &EnemyAnimationStates,
        resource_manager: &ResourceManager,
    ) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = resource_manager
            .textures
            .enemies
            .get(&EntSpriteKV::State(EntTypeKey {
                ty: entity_state.clone(),
                anim_ty: animation_state.clone(),
            }))
        {
            return match x {
                EntSpriteKV::Handle(h) => Some(h.clone()),
                _ => None,
            };
        }

        None
    }
}

impl Default for Enemies {
    fn default() -> Self {
        Self {}
    }
}
