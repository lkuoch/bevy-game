use bevy::prelude::*;

#[derive(Component)]
pub enum Collider {
    // Player in front
    NoneBackground,

    // Player at back
    NoneForeground,

    Wall,
    Ceiling,
    Ground,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Hash)]
pub enum BackgroundType {
    Blue,
    Brown,
    Gray,
    Green,
    Pink,
    Purple,
    Yellow,
}
