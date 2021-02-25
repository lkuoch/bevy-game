use crate::debug::systems::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup.system())
            .add_system(text_update.system());
    }
}
