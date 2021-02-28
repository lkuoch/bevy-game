#![feature(format_args_capture)]

mod plugins;

use crate::plugins::{camera, coordinator, core, debug, input, player, world};
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(core::CorePlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(coordinator::CoordinatorPlugin)
        .add_plugin(world::WorldPlugin)
        .add_plugin(player::PlayerPlugin)
        .run();
}
