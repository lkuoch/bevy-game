use crate::{
    core::state_machine::Machine,
    plugins::{
        animation::{
            components::{EntSpriteKV, EntTypeKey},
            traits::Animatable,
        },
        player::components::*,
        resource_manager::components::ResourceManager,
    },
};
use bevy::prelude::*;

impl PlayerState {
    pub fn movement_state_logic(
        current_state: &MovementState,
        command: &PlayerCommand,
    ) -> Option<MovementState> {
        match command {
            PlayerCommand::Idle => Some(MovementState::Idle),
            PlayerCommand::Jump => Some(MovementState::Jumping),
            PlayerCommand::OnGround => Some(MovementState::Standing),
            PlayerCommand::Crouch => Some(MovementState::Crouching),
            PlayerCommand::Standup => Some(MovementState::Standing),
            PlayerCommand::MovementComplete => Some(MovementState::Grounded),
            PlayerCommand::Movement(dir) => match dir {
                PlayerMovementDirection::Right => Some(MovementState::Moving(*dir)),
                PlayerMovementDirection::Left => Some(MovementState::Moving(*dir)),
            },
            _ => None,
        }
    }

    pub fn transformation_state_logic(
        current_state: &PlayerTransformationState,
        command: &PlayerCommand,
    ) -> Option<PlayerTransformationState> {
        let transform = || -> PlayerTransformationState {
            match current_state {
                PlayerTransformationState::MaskDude => PlayerTransformationState::NinjaFrog,
                PlayerTransformationState::NinjaFrog => PlayerTransformationState::PinkMan,
                PlayerTransformationState::PinkMan => PlayerTransformationState::VirtualGuy,
                PlayerTransformationState::VirtualGuy => PlayerTransformationState::MaskDude,
            }
        };

        match command {
            PlayerCommand::Transform => Some(transform()),
            _ => None,
        }
    }
}

impl Machine<MovementState> {
    pub fn enqueue(&mut self, command: PlayerCommand) {
        if let Some(new_state) = PlayerState::movement_state_logic(&self.state, &command) {
            self.state = new_state;
        }
    }
}

impl Machine<PlayerTransformationState> {
    pub fn enqueue(&mut self, command: PlayerCommand) {
        if let Some(new_state) = PlayerState::transformation_state_logic(&self.state, &command) {
            self.state = new_state;
        }
    }
}

impl Animatable<AnimationType> for PlayerState {
    fn get_texture_handle_from_state(
        &self,
        handle: Handle<TextureAtlas>,
        resource_manager: &ResourceManager,
    ) -> Option<AnimationType> {
        if let Some(x) = resource_manager
            .textures
            .players
            .get(&EntSpriteKV::Handle(handle))
        {
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
        resource_manager: &ResourceManager,
    ) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = resource_manager
            .textures
            .players
            .get(&EntSpriteKV::State(EntTypeKey {
                ty: self.transformation.state,
                anim_ty: state,
            }))
        {
            match x {
                EntSpriteKV::Handle(h) => Some(h.clone()),
                _ => None,
            }
        } else {
            None
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

impl Default for Machine<PlayerTransformationState> {
    fn default() -> Self {
        Self {
            state: PlayerTransformationState::default(),
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            movement: Machine::<MovementState>::default(),
            transformation: Machine::<PlayerTransformationState>::default(),
        }
    }
}

impl Default for AnimationType {
    fn default() -> Self {
        Self::Idle
    }
}

impl Default for PlayerTransformationState {
    fn default() -> Self {
        Self::MaskDude
    }
}
