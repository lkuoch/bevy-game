use crate::player::vars::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub(super) type PlayerSpriteMap<T> = HashMap<PlayerSpriteKV<T>, PlayerSpriteKV<T>>;

#[derive(Debug, Clone)]
pub(super) struct PlayerState {
    pub dir: DirState,
    pub attack: AttackState,
    pub jump: JumpState,
    pub movement: MovementState,

    // Textures for mask dude
    pub current_sprite: PlayerType,
    pub previous_sprite: PlayerType,

    // Can I make this all generic?
    pub mask_dude_textures: PlayerSpriteMap<mask_dude::States>,
    pub ninja_frog_textures: PlayerSpriteMap<ninja_frog::States>,
    pub pink_man_textures: PlayerSpriteMap<pink_man::States>,
    pub virtual_guy_textures: PlayerSpriteMap<virtual_guy::States>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum PlayerType {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum PlayerSpriteKV<T> {
    State(T),
    Handle(Handle<TextureAtlas>),
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

    pub fn jump(&mut self) {
        self.jump = JumpState::Jumping;
    }

    pub fn land(&mut self) {
        self.jump = JumpState::None;
    }

    pub fn reset_movement(&mut self) {
        self.movement = MovementState::None;
    }

    pub fn get_mask_dude_state_from_texture_handle(
        &self,
        animation: mask_dude::States,
    ) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = self
            .mask_dude_textures
            .get(&PlayerSpriteKV::State(animation))
        {
            match x {
                PlayerSpriteKV::Handle(h) => Some(h.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_texture_handle_from_mask_dude_state(
        &self,
        handle: Handle<TextureAtlas>,
    ) -> Option<mask_dude::States> {
        if let Some(x) = self.mask_dude_textures.get(&PlayerSpriteKV::Handle(handle)) {
            match x {
                PlayerSpriteKV::State(s) => Some(*s),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn transform_next(&mut self) {
        // self.current_sprite = PlayerType::NinjaFrog;
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

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            dir: DirState::Right,
            attack: AttackState::None,
            jump: JumpState::None,
            movement: MovementState::None,

            previous_sprite: PlayerType::MaskDude,
            current_sprite: PlayerType::MaskDude,
            mask_dude_textures: PlayerSpriteMap::<mask_dude::States>::new(),
            ninja_frog_textures: PlayerSpriteMap::<ninja_frog::States>::new(),
            pink_man_textures: PlayerSpriteMap::<pink_man::States>::new(),
            virtual_guy_textures: PlayerSpriteMap::<virtual_guy::States>::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum MovementState {
    None,
    Moving(DirState),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum DirState {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum JumpState {
    None,
    Jumping,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum AttackState {
    None,
    Preparing,
    Attacking,
}
