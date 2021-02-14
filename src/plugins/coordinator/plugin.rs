use bevy::prelude::*;

pub struct CoordinatorPlugin;
impl Plugin for CoordinatorPlugin {
    fn build(&self, app: &mut AppBuilder) {
      // Purpose of this plugin is to be global game coordinator.
      // Should be able to read track game state and spawn enemies

      // TODO
      // - Game manager can spawn sprites of objects
      // - Simple game logic
    }
}
