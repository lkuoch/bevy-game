use bevy::prelude::*;

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

impl Animatable<PlayerTypeStates, PlayerAnimationStates> for FromPlayer {
    fn get_texture_handle_from_state(
        &self,
        handle: &Handle<TextureAtlas>,
        resource_manager: &ResourceManager,
    ) -> Option<PlayerAnimationStates> {
        if let Some(x) = resource_manager
            .textures
            .players
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
        entity_state: &PlayerTypeStates,
        animation_state: &PlayerAnimationStates,
        resource_manager: &ResourceManager,
    ) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = resource_manager
            .textures
            .players
            .get(&EntSpriteKV::State(EntTypeKey {
                ty: entity_state.clone(),
                anim_ty: animation_state.clone(),
            }))
        {
            return match x {
                EntSpriteKV::Handle(h) => Some(h.clone()),
                _ => None,
            };
        } else {
            None
        }
    }
}

impl PlayerMovementState {
    pub fn get(&self) -> PlayerMovementStates {
        self.machine.state
    }

    pub fn enqueue(&mut self, command: PlayerCommands) {
        if let Some(new_state) =
            PlayerMovementState::movement_state_logic(&self.machine.state, &command)
        {
            self.machine.state = new_state;
        }
    }

    fn movement_state_logic(
        current_state: &PlayerMovementStates,
        command: &PlayerCommands,
    ) -> Option<PlayerMovementStates> {
        match command {
            PlayerCommands::Idle => Some(PlayerMovementStates::Idle),
            PlayerCommands::Jump => Some(PlayerMovementStates::Jumping),
            PlayerCommands::OnGround => Some(PlayerMovementStates::Standing),
            PlayerCommands::Crouch => Some(PlayerMovementStates::Crouching),
            PlayerCommands::Standup => Some(PlayerMovementStates::Standing),
            PlayerCommands::MovementComplete => Some(PlayerMovementStates::Grounded),
            PlayerCommands::Movement(dir) => match dir {
                PlayerMovementDirection::Right => Some(PlayerMovementStates::Moving(*dir)),
                PlayerMovementDirection::Left => Some(PlayerMovementStates::Moving(*dir)),
            },
            _ => None,
        }
    }
}

impl PlayerTypeState {
    pub fn get(&self) -> PlayerTypeStates {
        self.machine.state
    }

    pub fn enqueue(&mut self, command: PlayerCommands) {
        if let Some(new_state) =
            PlayerTypeState::transformation_state_logic(&self.machine.state, &command)
        {
            self.machine.state = new_state;
        }
    }

    fn transformation_state_logic(
        current_state: &PlayerTypeStates,
        command: &PlayerCommands,
    ) -> Option<PlayerTypeStates> {
        let transform = || -> PlayerTypeStates {
            match current_state {
                PlayerTypeStates::MaskDude => PlayerTypeStates::NinjaFrog,
                PlayerTypeStates::NinjaFrog => PlayerTypeStates::PinkMan,
                PlayerTypeStates::PinkMan => PlayerTypeStates::VirtualGuy,
                PlayerTypeStates::VirtualGuy => PlayerTypeStates::MaskDude,
            }
        };

        match command {
            PlayerCommands::Transform => Some(transform()),
            _ => None,
        }
    }
}

impl PlayerAnimationState {
    pub fn get(&self) -> PlayerAnimationStates {
        self.machine.state
    }

    pub fn get_movement_state_animation(
        &self,
        player_movement_state: &PlayerMovementStates,
    ) -> PlayerAnimationStates {
        match player_movement_state {
            PlayerMovementStates::Idle
            | PlayerMovementStates::Grounded
            | PlayerMovementStates::Standing => PlayerAnimationStates::Idle,
            PlayerMovementStates::Moving(_) => PlayerAnimationStates::Run,
            PlayerMovementStates::Jumping => PlayerAnimationStates::DoubleJump,
            _ => PlayerAnimationStates::Idle,
        }
    }
}

impl Default for PlayerMovementState {
    fn default() -> Self {
        Self {
            machine: Machine::<PlayerMovementStates> {
                state: PlayerMovementStates::default(),
            },
        }
    }
}

impl Default for PlayerTypeState {
    fn default() -> Self {
        Self {
            machine: Machine::<PlayerTypeStates> {
                state: PlayerTypeStates::default(),
            },
        }
    }
}

impl Default for PlayerAnimationState {
    fn default() -> Self {
        Self {
            machine: Machine::<PlayerAnimationStates> {
                state: PlayerAnimationStates::default(),
            },
        }
    }
}

impl Default for PlayerMovementStates {
    fn default() -> Self {
        Self::Idle
    }
}

impl Default for PlayerTypeStates {
    fn default() -> Self {
        Self::MaskDude
    }
}

impl Default for PlayerAnimationStates {
    fn default() -> Self {
        Self::Idle
    }
}
