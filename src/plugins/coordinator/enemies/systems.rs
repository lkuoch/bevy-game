use crate::{
    coordinator::enemies::{components::*, vars::*},
    plugins::core::components::*,
};
use bevy::prelude::*;

pub fn enemies_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut enemies: ResMut<Enemies>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for enemy in enemies::ENEMY_ANIMATIONS.iter() {
        for anim in enemy.animation_states.iter() {
            let path = format!("{}{}", enemy.root_path.to_owned(), anim.path.to_owned());

            let texture_handle: Handle<Texture> = asset_server.load(&path[..]);
            let handle = texture_atlases.add(TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(anim.tile_size.0, anim.tile_size.1),
                anim.frames,
                1,
            ));

            enemies.textures.insert(
                EnemySpriteMapKey::State(anim.kv),
                EntSpriteKV::Handle(handle.clone()),
            );

            enemies.textures.insert(
                EntSpriteKV::Handle(handle.clone()),
                EntSpriteKV::State(anim.kv.anim_ty),
            );
        }
    }

    let default_enemy = EntTypeKey {
        ty: EnemyType::Turtle,
        anim_ty: AnimationType::Idle,
    };

    // Let's just spawn common enemy
    if let Some(EntSpriteKV::Handle(enemy)) =
        enemies.textures.get(&EntSpriteKV::State(default_enemy))
    {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: enemy.clone(),
                transform: Transform::from_xyz(150., 0., 0.)
                    .mul_transform(Transform::from_scale(Vec3::splat(2.))),
                ..Default::default()
            })
            .with(EnemyTag {
                current_sprite: default_enemy.ty,
            })
            .with(AnimatableTag)
            .with(Timer::from_seconds(0.1, true));
    }
}
