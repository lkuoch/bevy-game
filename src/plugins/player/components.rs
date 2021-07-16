use crate::core::state_machine::Machine;

#[derive(Debug)]
pub struct PlayerTag;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayerState {
    pub movement: Machine<MovementState>,
    pub transformation: Machine<PlayerTransformationState>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlayerTransformationState {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum AnimationType {
    Idle,
    DoubleJump,
    Fall,
    Hit,
    Jump,
    Run,
    WallJump,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MovementState {
    Idle,
    Jumping,
    Standing,
    Crouching,
    Grounded,
    Moving(PlayerMovementDirection),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerMovementDirection {
    Right,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerCommand {
    Idle,
    Jump,
    OnGround,
    Crouch,
    Standup,
    Transform,
    MovementComplete,
    Movement(PlayerMovementDirection),
}
