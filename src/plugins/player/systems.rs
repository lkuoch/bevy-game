use crate::plugins::{
    animation::{components::*, traits::*},
    input::components::*,
    player::{components::*, vars::*},
    resource_manager::components::{ResourceManager, SpriteMapKey},
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn setup_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut resource_manager: ResMut<ResourceManager>,
) {
    for player in player::PLAYER_ANIMATIONS.iter() {
        for anim in player.animation_states.iter() {
            let path = format!("{}{}", player.root_path.to_owned(), anim.path.to_owned());

            let texture_handle: Handle<Texture> = asset_server.load(&path[..]);
            let handle = texture_atlases.add(TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(anim.tile_size.0, anim.tile_size.1),
                anim.frames,
                1,
            ));

            resource_manager.textures.players.insert(
                SpriteMapKey::State(anim.kv),
                EntSpriteKV::Handle(handle.clone()),
            );

            resource_manager.textures.players.insert(
                EntSpriteKV::Handle(handle.clone()),
                EntSpriteKV::State(anim.kv.anim_ty),
            );

            // Spawn player
            if anim.kv.ty == PlayerTypeStates::default() {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: handle.clone(),
                        transform: Transform::from_scale(Vec3::splat(2.5)),
                        ..Default::default()
                    })
                    .insert(FromPlayer)
                    .insert(AnimatableTag)
                    .insert(Timer::from_seconds(0.1, true));
            }
        }
    }
}

pub fn handle_input_event_system(
    mut event_reader: EventReader<InputEvent>,
    mut player_movement_state: ResMut<PlayerMovementState>,
    mut player_type_state: ResMut<PlayerTypeState>,
) {
    for event in event_reader.iter() {
        if let Some(key) = event.pressed {
            match key {
                KeyCode::A => {
                    player_movement_state
                        .enqueue(PlayerCommands::Movement(PlayerMovementDirection::Left));
                }
                KeyCode::D => {
                    player_movement_state
                        .enqueue(PlayerCommands::Movement(PlayerMovementDirection::Right));
                }
                KeyCode::Space => {
                    player_movement_state.enqueue(PlayerCommands::Jump);
                }
                KeyCode::T => player_type_state.enqueue(PlayerCommands::Transform),
                _ => {}
            }
        }

        if let Some(key) = event.released {
            match key {
                KeyCode::A => {
                    player_movement_state.enqueue(PlayerCommands::MovementComplete);
                }
                KeyCode::D => {
                    player_movement_state.enqueue(PlayerCommands::MovementComplete);
                }
                _ => {}
            }
        }
    }
}

pub fn player_movement_state_system(
    time: Res<Time>,
    player_movement_state: Res<PlayerMovementState>,
    mut query: Query<&mut Transform, With<FromPlayer>>,
) {
    query.for_each_mut(|(mut transform)| {
        if let PlayerMovementStates::Moving(direction) = player_movement_state.get() {
            let dir = match direction {
                PlayerMovementDirection::Left => {
                    transform.rotation = Quat::from_rotation_y(PI);
                    -1.0
                }
                PlayerMovementDirection::Right => {
                    transform.rotation = Quat::default();
                    1.0
                }
            };

            transform.translation.x += time.delta_seconds() * dir * player::BASE_SPEED;
        }
    });
}

pub fn player_type_state_system(
    resource_manager: Res<ResourceManager>,
    player_type_state: Res<PlayerTypeState>,
    mut query: Query<&mut Handle<TextureAtlas>, (With<FromPlayer>, Changed<PlayerTypeState>)>,
) {
    query.for_each_mut(|mut texture_handle| {
        if let Some(EntSpriteKV::Handle(curr_texture_handle)) = resource_manager
            .textures
            .players
            .get(&EntSpriteKV::State(EntTypeKey {
                ty: player_type_state.get(),
                anim_ty: PlayerAnimationStates::Idle,
            }))
        {
            *texture_handle = curr_texture_handle.clone();
        }
    });
}

pub fn change_animation_system(
    player_type_state: Res<PlayerTypeState>,
    player_movement_state: Res<PlayerMovementState>,
    player_animation_state: Res<PlayerAnimationState>,
    resource_manager: Res<ResourceManager>,
    mut query: Query<(&FromPlayer, &mut Handle<TextureAtlas>)>,
) {
    query.for_each_mut(|(from_player, mut texture)| {
        let entity_state = &player_type_state.get();

        *texture = from_player
            .get_state_from_texture_handle(
                entity_state,
                &player_animation_state.get_movement_state_animation(&player_movement_state.get()),
                &resource_manager,
            )
            .expect(&format!("Missing animation {entity_state:?}"));
    });
}

pub fn animation_lifecycle_system(
    resource_manager: Res<ResourceManager>,
    query: Query<&FromPlayer>,
    mut events: EventReader<AnimEvent<Handle<TextureAtlas>>>,
    mut player_movement_state: ResMut<PlayerMovementState>,
) {
    for event in events.iter() {
        match event {
            AnimEvent::Start(_handle) => {}
            // Finished animations
            AnimEvent::Finish(handle) => {
                query.for_each(|from_player| {
                    if let Some(animation_state) =
                        from_player.get_texture_handle_from_state(&handle, &resource_manager)
                    {
                        match animation_state {
                            PlayerAnimationStates::DoubleJump | PlayerAnimationStates::Jump => {
                                // Reset player state
                                player_movement_state.enqueue(PlayerCommands::Idle);
                            }

                            _ => {}
                        }
                    }
                });
            }
        }
    }
}
