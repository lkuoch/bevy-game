use crate::core::state_machine::Machine;

#[derive(Debug)]
pub struct PlayerTag;

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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayerMovementState {
    pub machine: Machine<PlayerMovementStates>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerMovementStates {
    Idle,
    Jumping,
    Standing,
    Crouching,
    Grounded,
    Moving(PlayerMovementDirection),
}

// Tracks player types
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayerTypeState {
    pub machine: Machine<PlayerTypeStates>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlayerTypeStates {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}

// Tracks animations
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayerAnimationState {
    pub machine: Machine<PlayerAnimationStates>,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum PlayerAnimationStates {
    Idle,
    DoubleJump,
    Fall,
    Hit,
    Jump,
    Run,
    WallJump,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerMovementDirection {
    Right,
    Left,
}
