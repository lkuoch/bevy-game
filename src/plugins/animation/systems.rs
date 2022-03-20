use bevy::{app::Events, prelude::*};

use crate::plugins::animation::components::*;

pub fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut events: ResMut<Events<AnimEvent<Handle<TextureAtlas>>>>,
    mut query: Query<(
        &WithAnimation,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) { 
    query.for_each_mut(|(_, mut timer, mut sprite, texture_atlas_handle)| {
        timer.tick(time.delta());

        if timer.finished() {
            // Anim start
            if sprite.index == 0 {
                events.send(AnimEvent::Start(texture_atlas_handle.clone()));
            }

            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();

            // Anim finish
            if sprite.index as usize == texture_atlas.textures.len() - 1 {
                events.send(AnimEvent::Finish(texture_atlas_handle.clone()));
            }
        }
    });
}
