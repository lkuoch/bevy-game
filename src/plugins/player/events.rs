use crate::player::vars::mask_dude as MaskDude;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq)]
pub(super) struct PlayerEvent {
    pub anim_start: Option<MaskDude::States>,
    pub anim_finish: Option<MaskDude::States>,
}
