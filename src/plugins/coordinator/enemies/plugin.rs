use crate::coordinator::enemies::{components::*, systems::*};
use bevy::prelude::*;

pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Enemies::default())
            .add_startup_system(setup_system);
    }
}
