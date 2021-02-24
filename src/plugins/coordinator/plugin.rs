use crate::coordinator::enemies::{components::*, systems::*};
use bevy::prelude::*;

pub struct CoordinatorPlugin;
impl Plugin for CoordinatorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Enemies::default())
            .add_startup_system(enemies_setup.system());
        // Purpose of this plugin is to be global game coordinator.
        // Should be able to read track game state and spawn enemies and items

        // TODO
        // - Game manager can spawn sprites of objects
        // - Simple game logic
    }
}
