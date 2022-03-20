use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntSpriteKV<T> {
    State(T),
    Handle(Handle<TextureAtlas>),
}

pub struct AnimationState<'a, T1, T2> {
    pub kv: EntTypeKey<T1, T2>,
    pub frames: usize,
    pub tile_size: (f32, f32),
    pub path: &'a str,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EntTypeKey<T1, T2> {
    pub anim_ty: T1,
    pub ty: T2,
}

pub struct EntList<'a, T1, T2> {
    pub animation_states: &'a [AnimationState<'a, T1, T2>],
    pub root_path: &'a str,
}

// Animation events
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub enum AnimEvent<T> {
    Start(T),
    Finish(T),
}

#[derive(Component)]
pub struct WithAnimation;
