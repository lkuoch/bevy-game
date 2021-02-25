use bevy::prelude::*;
use std::collections::HashMap;

pub(super) type PlayerSpriteMap<T> = HashMap<PlayerSpriteKV<PlayerTypeKey<T>>, PlayerSpriteKV<T>>;
pub(super) type PlayerSpriteMapKey = PlayerSpriteKV<PlayerTypeKey<States>>;

pub struct Player;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum PlayerSpriteKV<T> {
    State(T),
    Handle(Handle<TextureAtlas>),
}

pub struct AnimationState<'a, T> {
    pub kv: PlayerTypeKey<T>,
    pub frames: usize,
    pub path: &'a str,
}

pub struct PlayerList<'a, T> {
    pub animation_states: &'a [AnimationState<'a, T>],
    pub root_path: &'a str,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PlayerTypeKey<T> {
    pub state: T,
    pub ty: PlayerType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlayerType {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum States {
    Idle,
    DoubleJump,
    Fall,
    Hit,
    Jump,
    Run,
    WallJump,
}

#[derive(Debug, Clone)]
pub(super) struct PlayerState {
    pub dir: DirState,
    pub attack: AttackState,
    pub jump: JumpState,
    pub movement: MovementState,

    pub current_sprite: PlayerType,
    pub previous_sprite: PlayerType,

    pub textures: PlayerSpriteMap<States>,
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

    pub fn get_texture_handle_from_state(&self, handle: Handle<TextureAtlas>) -> Option<States> {
        if let Some(x) = self.textures.get(&PlayerSpriteKV::Handle(handle)) {
            match x {
                PlayerSpriteKV::State(s) => Some(*s),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_state_from_texture_handle(&self, state: States) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = self.textures.get(&PlayerSpriteKV::State(PlayerTypeKey {
            ty: self.current_sprite,
            state,
        })) {
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
            textures: PlayerSpriteMap::<States>::new(),
        }
    }
}
