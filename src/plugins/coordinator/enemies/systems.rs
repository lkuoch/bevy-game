use crate::plugins::{
    animation::{components::*, traits::*},
    coordinator::{
        components::CoordinatorCommands,
        enemies::{components::*, vars::*},
    },
    input::components::InputEvent,
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
        ty: EnemyTypeStates::Turtle,
        anim_ty: EnemyAnimationStates::Idle,
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
            .insert(EnemyTag)
            .insert(EnemyTypeState::default())
            .insert(AnimatableTag)
            .insert(Timer::from_seconds(0.1, true));
    }
}

pub fn handle_input_event_system(
    mut event_reader: EventReader<InputEvent>,
    mut query: Query<&mut EnemyTypeState>,
) {
    for event in event_reader.iter() {
        if let Some(key) = event.pressed {
            if let Ok(mut enemy_type_state) = query.single_mut() {
                match key {
                    KeyCode::R => enemy_type_state
                        .enqueue(CoordinatorCommands::EnemyTransform(EnemyTypeStates::Bat)),
                    _ => {}
                }
            }
        }
    }
}

pub fn change_animation_system(
    resource_manager: Res<ResourceManager>,
    mut query: Query<(&EnemyTag, &EnemyTypeState, &mut Handle<TextureAtlas>)>,
) {
    query.for_each_mut(|(enemy_tag, enemy_type_state, mut texture)| {
        let entity_state = &enemy_type_state.get();

        *texture = enemy_tag
            .get_state_from_texture_handle(
                entity_state,
                &EnemyTypeStates::get_default_animation(entity_state),
                &resource_manager,
            )
            .expect(&format!("Missing animation {entity_state:?}"));
    });
}
