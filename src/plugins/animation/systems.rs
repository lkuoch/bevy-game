use std::time::Duration;

use bevy::{app::Events, prelude::*};

use crate::plugins::animation::components::*;

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut events: ResMut<Events<AnimEvent<Handle<TextureAtlas>>>>,
    mut query: Query<(
        &AnimatableTag,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    query.for_each_mut(|(_, mut timer, mut sprite, texture_atlas_handle)| {
        timer.tick(Duration::from_secs_f32(time.delta_seconds()));
        if timer.finished() {
            // Anim start
            if sprite.index == 0 {
                events.send(AnimEvent::Start(texture_atlas_handle.clone()));
            }

            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;

            // Anim finish
            if sprite.index as usize == texture_atlas.textures.len() - 1 {
                events.send(AnimEvent::Finish(texture_atlas_handle.clone()));
            }
        }
    });
}
