pub use defs::*;

mod defs {
    use crate::core::state_machine::Machine;

    #[derive(Debug)]
    pub struct PlayerTag;

    #[derive(Debug, Clone)]
    pub struct Player {
        pub state: PlayerState,

        pub current_sprite: PlayerType,
        pub previous_sprite: PlayerType,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct PlayerState {
        pub movement: Machine<MovementState>,
        pub transformation: Machine<PlayerType>,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum PlayerType {
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
        Jump,
        OnGround,
        Crouch,
        Standup,
        Transform,
        MovementComplete,
        Movement(PlayerMovementDirection),
    }
}

mod impls {
    use super::*;
    use crate::{
        core::state_machine::Machine,
        plugins::{
            animation::{
                components::{EntSpriteKV, EntTypeKey},
                traits::Animatable,
            },
            resource_manager::components::ResourceManager,
        },
    };
    use bevy::prelude::*;

    impl Player {
        pub fn transform_next(&mut self) {
            match self.current_sprite {
                PlayerType::MaskDude => {
                    self.previous_sprite = PlayerType::MaskDude;
                    self.current_sprite = PlayerType::NinjaFrog;
                }
                PlayerType::NinjaFrog => {
                    self.previous_sprite = PlayerType::NinjaFrog;
                    self.current_sprite = PlayerType::PinkMan;
                }

                PlayerType::PinkMan => {
                    self.previous_sprite = PlayerType::PinkMan;
                    self.current_sprite = PlayerType::VirtualGuy;
                }
                PlayerType::VirtualGuy => {
                    self.previous_sprite = PlayerType::VirtualGuy;
                    self.current_sprite = PlayerType::MaskDude;
                }
            }
        }
    }

    impl Animatable<AnimationType> for Player {
        fn get_texture_handle_from_state(
            &self,
            handle: Handle<TextureAtlas>,
            resource_manager: &ResourceManager,
        ) -> Option<AnimationType> {
            if let Some(x) = resource_manager
                .textures
                .player
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
                .player
                .get(&EntSpriteKV::State(EntTypeKey {
                    ty: self.current_sprite,
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

    impl Machine<MovementState> {
        pub fn enqueue(&mut self, command: PlayerCommand) {
            match command {
                PlayerCommand::Jump => self.state = MovementState::Jumping,
                PlayerCommand::OnGround => self.state = MovementState::Standing,
                PlayerCommand::Crouch => self.state = MovementState::Crouching,
                PlayerCommand::Standup => self.state = MovementState::Standing,
                PlayerCommand::MovementComplete => self.state = MovementState::Grounded,
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

    impl Default for Player {
        fn default() -> Self {
            Self {
                state: PlayerState::default(),

                previous_sprite: PlayerType::MaskDude,
                current_sprite: PlayerType::MaskDude,
            }
        }
    }

    impl Default for AnimationType {
        fn default() -> Self {
            Self::Idle
        }
    }

    impl Default for PlayerType {
        fn default() -> Self {
            Self::MaskDude
        }
    }
}
