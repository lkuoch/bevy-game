use crate::coordinator::enemies;
use bevy::prelude::*;

pub struct CoordinatorPlugin;
impl Plugin for CoordinatorPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Figure out how to make animation system genric
        // Purpose of this plugin is to be global game coordinator.
        // Should be able to read track game state and spawn enemies and items

        // TODO
        // - Game manager can spawn sprites of objects
        // - Simple game logic
        app.add_plugin(enemies::EnemiesPlugin);
    }
}
