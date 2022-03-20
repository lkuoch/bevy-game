use crate::world::{components::*, vars::*};
use bevy::prelude::*;

pub fn setup_system(mut commands: Commands) {
    let bounds = Vec2::new(1000.0, 700.0);

    commands
        // left
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(world::WALL_THICKNESS, bounds.y + world::WALL_THICKNESS, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::PINK,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Wall)
        .commands()
        // right
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(world::WALL_THICKNESS, bounds.y + world::WALL_THICKNESS, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::PINK,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Wall)
        .commands()
        // bottom
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + world::WALL_THICKNESS, world::WALL_THICKNESS, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::PINK,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Ground)
        .commands()
        // top
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + world::WALL_THICKNESS, world::WALL_THICKNESS, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::PINK,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Ceiling);
}
