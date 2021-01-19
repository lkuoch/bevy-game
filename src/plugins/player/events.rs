use crate::player::vars::player_common;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq)]
pub(super) struct AnimEvent {
    pub anim_start: Option<player_common::States>,
    pub anim_finish: Option<player_common::States>,
}
