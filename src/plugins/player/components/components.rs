pub struct PlayerTag;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlayerType {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}

impl Default for PlayerType {
    fn default() -> Self {
        Self::MaskDude
    }
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

impl Default for AnimationType {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MovementState {
    None,
    Moving(DirState),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DirState {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JumpState {
    None,
    Jumping,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttackState {
    None,
    Preparing,
    Attacking,
}
