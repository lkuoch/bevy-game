use crate::{
    common::components::*,
    coordinator::enemies::{components::*, vars::*},
};
use bevy::prelude::*;

pub fn enemies_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut enemies: ResMut<Enemies>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for enemy in enemies::ENEMY_LIST.iter() {
        for anim in enemy.animation_states.iter() {
            let path = format!("{}{}", enemy.root_path.to_owned(), anim.path.to_owned());

            let texture_handle: Handle<Texture> = asset_server.load(&path[..]);
            let handle = texture_atlases.add(TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(32.0, 32.0),
                anim.frames,
                1,
            ));

            enemies.textures.insert(
                EnemySpriteMapKey::State(anim.kv),
                EntSpriteKV::Handle(handle.clone()),
            );

            enemies.textures.insert(
                EntSpriteKV::Handle(handle.clone()),
                EntSpriteKV::State(anim.kv.state),
            );
        }
    }

    // Let's just spawn angry pig
    if let Some(EntSpriteKV::Handle(enemy)) =
        enemies.textures.get(&EntSpriteKV::State(EntTypeKey {
            ty: EnemyType::AngryPig,
            state: States::Idle,
        }))
    {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: enemy.clone(),
                transform: Transform::from_xyz(150., 0., 0.)
                    .mul_transform(Transform::from_scale(Vec3::splat(2.5))),
                ..Default::default()
            })
            .with(Enemy)
            .with(Timer::from_seconds(0.1, true));
    }
}

pub fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Enemy,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (_, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}
