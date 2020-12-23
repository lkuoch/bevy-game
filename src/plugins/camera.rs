use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(Camera2dBundle::default());
}
