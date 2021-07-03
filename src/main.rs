#![feature(format_args_capture)]

mod plugins;

use crate::plugins::{
    animation, camera, coordinator, debug, input, player, resource_manager, world,
};
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(resource_manager::ResourceManagerPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(coordinator::CoordinatorPlugin)
        .add_plugin(animation::AnimationPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(world::WorldPlugin)
        .add_plugin(player::PlayerPlugin)
        .run();
}
