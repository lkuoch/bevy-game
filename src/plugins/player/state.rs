#[derive(Debug, Clone, Copy)]
pub(super) struct PlayerState {
    attack: AttackState,
    jump: JumpState,
    movement: MovementState,
}

impl PlayerState {
    pub fn move_right(&mut self) {
        self.movement = MovementState::Moving(1);
    }

    pub fn move_left(&mut self) {
        self.movement = MovementState::Moving(-1);
    }

    pub fn reset_movement(&mut self) {
        self.movement = MovementState::None;
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            attack: AttackState::None,
            jump: JumpState::None,
            movement: MovementState::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum MovementState {
    None,
    Moving(i8),
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
