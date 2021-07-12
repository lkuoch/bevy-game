use crate::{core::state_machine::Machine, player::components::*};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayerState {
    pub movement: Machine<MovementState>,
    pub transformation: Machine<PlayerType>,
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
    Jump,
    OnGround,
    Crouch,
    Standup,
    Transform,
    MovementComplete,
    Movement(PlayerMovementDirection),
}

impl Machine<MovementState> {
    fn transition_movement(&mut self, new_state: MovementState) {
        self.state = new_state;
    }

    pub fn enqueue(&mut self, command: PlayerCommand) {
        match command {
            PlayerCommand::Jump => self.transition_movement(MovementState::Jumping),
            PlayerCommand::OnGround => self.transition_movement(MovementState::Standing),
            PlayerCommand::Crouch => self.transition_movement(MovementState::Crouching),
            PlayerCommand::Standup => self.transition_movement(MovementState::Standing),
            PlayerCommand::MovementComplete => self.transition_movement(MovementState::Grounded),
            PlayerCommand::Movement(dir) => match dir {
                PlayerMovementDirection::Right => self.state = MovementState::Moving(dir),
                PlayerMovementDirection::Left => self.state = MovementState::Moving(dir),
            },
            _ => {}
        }
    }
}

impl Default for Machine<MovementState> {
    fn default() -> Self {
        Self {
            state: MovementState::Idle,
        }
    }
}

impl Default for Machine<PlayerType> {
    fn default() -> Self {
        Self {
            state: PlayerType::default(),
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            movement: Machine::<MovementState>::default(),
            transformation: Machine::<PlayerType>::default(),
        }
    }
}
