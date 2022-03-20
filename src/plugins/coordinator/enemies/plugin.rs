use crate::coordinator::enemies::systems::*;
use bevy::prelude::*;

pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(handle_input_event_system)
            .add_system(change_animation_system);
    }
}
