#[derive(Debug, Clone, Copy)]
pub(super) struct PlayerState {
    pub dir: DirState,
    pub attack: AttackState,
    pub jump: JumpState,
    pub movement: MovementState,
}

impl PlayerState {
    pub fn move_right(&mut self) {
        self.movement = MovementState::Moving(DirState::Right);
        self.dir = DirState::Right;
    }

    pub fn move_left(&mut self) {
        self.movement = MovementState::Moving(DirState::Left);
        self.dir = DirState::Left;
    }

    pub fn reset_movement(&mut self) {
        self.movement = MovementState::None;
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            dir: DirState::Right,
            attack: AttackState::None,
            jump: JumpState::None,
            movement: MovementState::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum MovementState {
    None,
    Moving(DirState),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum DirState {
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
pub(super) enum JumpState {
    None,
    Jumping,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum AttackState {
    None,
    Preparing,
    Attacking,
}
