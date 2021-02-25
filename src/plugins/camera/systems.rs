use bevy::prelude::*;

pub fn setup(commands: &mut Commands) {
    commands
        .spawn(UiCameraBundle::default())
        .spawn(OrthographicCameraBundle::new_2d());
}
