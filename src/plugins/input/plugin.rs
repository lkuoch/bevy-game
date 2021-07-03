use crate::input::{components::*, systems::*};
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<InputEvent>()
            .add_system(keyboard_input_system);
    }
}
