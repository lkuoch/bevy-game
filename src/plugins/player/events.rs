use crate::player::vars::player;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq)]
pub(super) struct AnimEvent {
    pub anim_start: Option<player::States>,
    pub anim_finish: Option<player::States>,
}
