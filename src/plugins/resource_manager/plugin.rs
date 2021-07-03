use crate::resource_manager::components::*;
use bevy::prelude::*;

pub struct ResourceManagerPlugin;
impl Plugin for ResourceManagerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ResourceManager::default());
    }
}
