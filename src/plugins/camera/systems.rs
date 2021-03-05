use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands
        .spawn(UiCameraBundle::default())
        .spawn(OrthographicCameraBundle::new_2d());
}
