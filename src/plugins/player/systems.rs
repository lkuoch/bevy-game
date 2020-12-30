use crate::player::vars::mask_dude as MaskDude;
use crate::player::components::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub(super) fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut textures_map = HashMap::<MaskDude::States, Handle<TextureAtlas>>::new();

    for texture in MaskDude::TEXTURES {
        let texture_handle = asset_server.load(texture.path);

        let handle = texture_atlases.add(TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(32.0, 32.0),
            texture.frames,
            1,
        ));

        textures_map.insert(texture.state, handle);
    }

    if let Some(default_texture) = textures_map.get(&MaskDude::States::Idle) {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: default_texture.clone(),
                transform: Transform::from_scale(Vec3::splat(2.5)),
                ..Default::default()
            })
            .with(Timer::from_seconds(0.1, true))
            .with(Player {
                textures: textures_map,
            });
    }
}

pub(super) fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}
