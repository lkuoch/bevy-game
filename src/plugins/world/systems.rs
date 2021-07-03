use crate::world::{components::*, vars::*};
use bevy::prelude::*;

pub fn setup_system(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    if let Ok(wall_color) = Color::hex(world::WALL_COLOR_1) {
        let wall_material = materials.add(wall_color.into());
        let bounds = Vec2::new(1000.0, 700.0);

        commands
            // left
            .spawn_bundle(SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(
                    world::WALL_THICKNESS,
                    bounds.y + world::WALL_THICKNESS,
                )),
                ..Default::default()
            })
            .insert(Collider::Wall)
            .commands()
            // right
            .spawn_bundle(SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(bounds.x / 2.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(
                    world::WALL_THICKNESS,
                    bounds.y + world::WALL_THICKNESS,
                )),
                ..Default::default()
            })
            .insert(Collider::Wall)
            .commands()
            // bottom
            .spawn_bundle(SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
                sprite: Sprite::new(Vec2::new(
                    bounds.x + world::WALL_THICKNESS,
                    world::WALL_THICKNESS,
                )),
                ..Default::default()
            })
            .insert(Collider::Ground)
            .commands()
            // top
            .spawn_bundle(SpriteBundle {
                material: wall_material,
                transform: Transform::from_translation(Vec3::new(0.0, bounds.y / 2.0, 0.0)),
                sprite: Sprite::new(Vec2::new(
                    bounds.x + world::WALL_THICKNESS,
                    world::WALL_THICKNESS,
                )),
                ..Default::default()
            })
            .insert(Collider::Ceiling);
    }
}
