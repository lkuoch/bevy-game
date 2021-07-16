use crate::player::{components::*, systems::*};
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(PlayerMovementState::default())
            .insert_resource(PlayerTypeState::default())
            .insert_resource(PlayerAnimationState::default())
            .add_startup_system(setup_system)
            .add_system(handle_input_event_system)
            .add_system(player_movement_state_system)
            .add_system(player_type_state_system)
            .add_system(change_animation_system)
            .add_system(animation_lifecycle_system);
    }
}
