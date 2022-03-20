use bevy::prelude::*;

use crate::core::state_machine::Machine;

#[derive(Component)]
pub struct IsEnemy;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct EnemyTypeState {
    pub machine: Machine<EnemyTypeStates>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnemyTypeStates {
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
