use bevy::prelude::*;

use crate::core::state_machine::Machine;

#[derive(Component)]
pub struct IsPlayer;

// Possible player state machine commands
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerCommands {
    Idle,
    Jump,
    OnGround,
    Crouch,
    Standup,
    Transform,
    MovementComplete,
    Movement(PlayerMovementDirection),
}

// Tracks movement
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerMovementState {
    pub machine: Machine<PlayerMovementStates>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerMovementStates {
    Idle,
    Jumping,
    Standing,
    Crouching,
    Grounded,
    Moving(PlayerMovementDirection),
}

// Tracks player types
#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub struct PlayerTypeState {
    pub machine: Machine<PlayerTypeStates>,
}

#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerTypeStates {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}

// Tracks animations
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerAnimationState {
    pub machine: Machine<PlayerAnimationStates>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerAnimationStates {
    Idle,
    DoubleJump,
    Fall,
    Hit,
    Jump,
    Run,
    WallJump,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerMovementDirection {
    Right,
    Left,
}
