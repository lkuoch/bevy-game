use crate::player::components::*;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq)]
pub(super) struct AnimEvent {
    pub anim_start: Option<States>,
    pub anim_finish: Option<States>,
}
