use crate::player::{components::Player, systems::*};
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Player::default())
            .add_startup_system(setup_system)
            .add_system(handle_input_event_system)
            .add_system(observe_player_state_system)
            .add_system(change_animation_system)
            .add_system(handle_animation_system);
    }
}
