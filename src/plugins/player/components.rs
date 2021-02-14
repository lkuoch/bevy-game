use crate::player::vars::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub(super) type PlayerSpriteMap<T> = HashMap<PlayerSpriteKV<PlayerTypeKey<T>>, PlayerSpriteKV<T>>;
pub(super) type PlayerSpriteMapKey = PlayerSpriteKV<PlayerTypeKey<player_common::States>>;

pub struct Player;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum PlayerSpriteKV<T> {
    State(T),
    Handle(Handle<TextureAtlas>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum PlayerTypeKey<T> {
    MaskDude(T),
    NinjaFrog(T),
    PinkMan(T),
    VirtualGuy(T),
}

#[derive(Debug, Clone)]
pub(super) struct PlayerState {
    pub dir: DirState,
    pub attack: AttackState,
    pub jump: JumpState,
    pub movement: MovementState,

    pub current_sprite: PlayerType,
    pub previous_sprite: PlayerType,

    pub textures: PlayerSpriteMap<player_common::States>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerType {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
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

    pub fn get_texture_handle_from_state(
        &self,
        handle: Handle<TextureAtlas>,
    ) -> Option<player_common::States> {
        if let Some(x) = self.textures.get(&PlayerSpriteKV::Handle(handle)) {
            match x {
                PlayerSpriteKV::State(s) => Some(*s),
                _ => None,
            }
        } else {
            None
        }
    }

    fn get_player_type_key(
        &self,
        state: player_common::States,
    ) -> PlayerTypeKey<player_common::States> {
        match self.current_sprite {
            PlayerType::MaskDude => PlayerTypeKey::MaskDude(state),
            PlayerType::NinjaFrog => PlayerTypeKey::NinjaFrog(state),
            PlayerType::PinkMan => PlayerTypeKey::PinkMan(state),
            PlayerType::VirtualGuy => PlayerTypeKey::VirtualGuy(state),
        }
    }

    pub fn get_state_from_texture_handle(
        &self,
        state: player_common::States,
    ) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = self
            .textures
            .get(&PlayerSpriteKV::State(self.get_player_type_key(state)))
        {
            match x {
                PlayerSpriteKV::Handle(h) => Some(h.clone()),
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

            previous_sprite: PlayerType::MaskDude,
            current_sprite: PlayerType::MaskDude,

            // Stores all playable character textures
            textures: PlayerSpriteMap::<player_common::States>::new(),
        }
    }
}
