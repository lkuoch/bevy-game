mod plugin;
pub use plugin::CoordinatorPlugin;

pub use enemies::components::*;
pub mod components;
mod enemies;
mod systems;
mod vars;
