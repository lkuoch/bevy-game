#![feature(format_args_capture)]

mod plugins;
mod vars;

use crate::plugins::{camera, debug, player};
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .run();
}
