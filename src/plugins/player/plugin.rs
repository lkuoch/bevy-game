use crate::player::{events::*, state::PlayerState, systems::*};
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(PlayerState::default())
            .add_event::<AnimEvent>()
            .add_startup_system(setup.system())
            .add_system(animate_sprite_system.system())
            .add_system(handle_input_event.system())
            .add_system(react_player_state.system())
            // .add_system(change_animation.system())
            .add_system(handle_player_event.system());
    }
}
