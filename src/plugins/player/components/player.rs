use bevy::{prelude::Handle, sprite::TextureAtlas};

use crate::plugins::{
    animation::{components::*, traits::Animatable},
    player::components::components::*,
    resource_manager::components::ResourceManager,
};

#[derive(Debug, Clone)]
pub struct Player {
    pub dir: DirState,
    pub attack: AttackState,
    pub jump: JumpState,
    pub movement: MovementState,

    pub current_sprite: PlayerType,
    pub previous_sprite: PlayerType,
}

impl Player {
    pub fn move_right(&mut self) {
        self.movement = MovementState::Moving(DirState::Right);
        self.dir = DirState::Right;
    }

    pub fn move_left(&mut self) {
        self.movement = MovementState::Moving(DirState::Left);
        self.dir = DirState::Left;
    }

    pub fn jump(&mut self) {
        self.jump = JumpState::Jumping;
    }

    pub fn land(&mut self) {
        self.jump = JumpState::None;
    }

    pub fn reset_movement(&mut self) {
        self.movement = MovementState::None;
    }

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

impl Default for Player {
    fn default() -> Self {
        Self {
            dir: DirState::Right,
            attack: AttackState::None,
            jump: JumpState::None,
            movement: MovementState::None,

            previous_sprite: PlayerType::MaskDude,
            current_sprite: PlayerType::MaskDude,
        }
    }
}