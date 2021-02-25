use crate::input::events::InputEvent;
use bevy::prelude::*;

pub fn keyboard_input_system(inputs: Res<Input<KeyCode>>, mut events: ResMut<Events<InputEvent>>) {
    for just_pressed in inputs.get_just_pressed() {
        events.send(InputEvent {
            pressed: Some(*just_pressed),
            released: None,
        });
    }

    for just_released in inputs.get_just_released() {
        events.send(InputEvent {
            pressed: None,
            released: Some(*just_released),
        });
    }
}
