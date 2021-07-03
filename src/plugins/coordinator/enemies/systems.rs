use crate::plugins::{
    animation::components::*,
    coordinator::enemies::{components::*, vars::*},
    resource_manager::components::{ResourceManager, SpriteMapKey},
};
use bevy::prelude::*;

pub fn setup_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut resource_manager: ResMut<ResourceManager>,
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

            resource_manager.textures.enemies.insert(
                SpriteMapKey::State(anim.kv),
                EntSpriteKV::Handle(handle.clone()),
            );

            resource_manager.textures.enemies.insert(
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
    if let Some(EntSpriteKV::Handle(enemy)) = resource_manager
        .textures
        .enemies
        .get(&EntSpriteKV::State(default_enemy))
    {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: enemy.clone(),
                transform: Transform::from_xyz(150., 0., 0.)
                    .mul_transform(Transform::from_scale(Vec3::splat(2.))),
                ..Default::default()
            })
            .insert(EnemyTag {
                current_sprite: default_enemy.ty,
            })
            .insert(AnimatableTag)
            .insert(Timer::from_seconds(0.1, true));
    }
}
