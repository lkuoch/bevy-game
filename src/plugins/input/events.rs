use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Hash)]
pub struct InputEvent {
    pub pressed: Option<KeyCode>,
    pub released: Option<KeyCode>,
}
