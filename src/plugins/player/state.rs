use crate::player::vars::mask_dude as MaskDude;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(super) struct PlayerState {
    pub dir: DirState,
    pub attack: AttackState,
    pub jump: JumpState,
    pub movement: MovementState,

    // Textures for mask dude
    pub current_sprite: CurrentSpriteType,
    pub mask_dude_textures: HashMap<MaskDudeTextureKeyValue, MaskDudeTextureKeyValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum CurrentSpriteType {
    MaskDude,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum MaskDudeTextureKeyValue {
    State(MaskDude::States),
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
        animation: MaskDude::States,
    ) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = self
            .mask_dude_textures
            .get(&MaskDudeTextureKeyValue::State(animation))
        {
            match x {
                MaskDudeTextureKeyValue::Handle(h) => Some(h.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_texture_handle_from_mask_dude_state(
        &self,
        handle: Handle<TextureAtlas>,
    ) -> Option<MaskDude::States> {
        if let Some(x) = self
            .mask_dude_textures
            .get(&MaskDudeTextureKeyValue::Handle(handle))
        {
            match x {
                MaskDudeTextureKeyValue::State(s) => Some(*s),
                _ => None,
            }
        } else {
            None
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

            current_sprite: CurrentSpriteType::MaskDude,
            mask_dude_textures: HashMap::<MaskDudeTextureKeyValue, MaskDudeTextureKeyValue>::new(),
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
