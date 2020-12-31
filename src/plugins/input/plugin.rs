use crate::input::event::InputEvent;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<InputEvent>()
            .add_system(keyboard_input_system.system());
    }
}

fn keyboard_input_system(inputs: Res<Input<KeyCode>>, mut events: ResMut<Events<InputEvent>>) {
    for just_pressed in inputs.get_pressed() {
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
