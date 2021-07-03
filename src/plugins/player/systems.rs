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

            resource_manager.textures.player.insert(
                SpriteMapKey::State(anim.kv),
                EntSpriteKV::Handle(handle.clone()),
            );

            resource_manager.textures.player.insert(
                EntSpriteKV::Handle(handle.clone()),
                EntSpriteKV::State(anim.kv.anim_ty),
            );

            // Spawn player
            if anim.kv.ty == PlayerType::default() {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: handle.clone(),
                        transform: Transform::from_scale(Vec3::splat(2.5)),
                        ..Default::default()
                    })
                    .insert(PlayerTag)
                    .insert(AnimatableTag)
                    .insert(Timer::from_seconds(0.1, true));
            }
        }
    }
}

pub fn handle_input_event_system(
    mut event_reader: EventReader<InputEvent>,
    mut player_state: ResMut<Player>,
) {
    for event in event_reader.iter() {
        if let Some(key) = event.pressed {
            match key {
                KeyCode::A => {
                    player_state.move_left();
                }
                KeyCode::D => {
                    player_state.move_right();
                }
                KeyCode::Space => {
                    player_state.jump();
                }
                KeyCode::T => {
                    player_state.transform_next();
                }
                _ => {}
            }
        }

        if let Some(key) = event.released {
            match key {
                KeyCode::A => {
                    player_state.reset_movement();
                }
                KeyCode::D => {
                    player_state.reset_movement();
                }
                _ => {}
            }
        }
    }
}

pub fn handle_animation_system(
    resource_manager: Res<ResourceManager>,
    mut events: EventReader<AnimEvent<Handle<TextureAtlas>>>,
    mut player_state: ResMut<Player>,
) {
    for event in events.iter() {
        match event {
            AnimEvent::Start(_handle) => {}
            AnimEvent::Finish(handle) => {
                if let Some(state) =
                    player_state.get_texture_handle_from_state(handle.clone(), &resource_manager)
                {
                    match state {
                        AnimationType::DoubleJump => player_state.land(),
                        AnimationType::Idle => {}
                        AnimationType::Fall => {}
                        AnimationType::Hit => {}
                        AnimationType::Jump => {}
                        AnimationType::Run => {}
                        AnimationType::WallJump => {}
                    }
                }
            }
            AnimEvent::None => {}
        }
    }
}

pub fn observe_player_state_system(
    time: Res<Time>,
    resource_manager: Res<ResourceManager>,
    mut player_state: ResMut<Player>,
    mut query: Query<(&PlayerTag, &mut Transform, &mut Handle<TextureAtlas>)>,
) {
    query.for_each_mut(|(_, mut transform, mut current_texture_handle)| {
        // Movement
        if let MovementState::Moving(dir) = player_state.movement {
            let dir_val = match dir {
                DirState::Left => {
                    transform.rotation = Quat::from_rotation_y(PI);
                    -1.0
                }
                DirState::Right => {
                    transform.rotation = Quat::default();
                    1.0
                }
            };

            transform.translation.x += time.delta_seconds() * dir_val * player::BASE_SPEED;
        }

        // Transformation
        if player_state.previous_sprite != player_state.current_sprite {
            if let Some(EntSpriteKV::Handle(mask_dude_texture_handle)) = resource_manager
                .textures
                .player
                .get(&EntSpriteKV::State(EntTypeKey {
                    ty: player_state.current_sprite,
                    anim_ty: AnimationType::Idle,
                }))
            {
                *current_texture_handle = mask_dude_texture_handle.clone();
            }

            player_state.previous_sprite = player_state.current_sprite.clone();
        }
    });
}

pub fn change_animation_system(
    player_state: Res<Player>,
    resource_manager: Res<ResourceManager>,
    mut query: Query<(&PlayerTag, &mut Handle<TextureAtlas>)>,
) {
    query.for_each_mut(|(_, mut current_texture_atlas_handle)| {
        if player_state.movement != MovementState::None {
            if let Some(new_anim_handle) =
                player_state.get_state_from_texture_handle(AnimationType::Run, &resource_manager)
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if player_state.jump != JumpState::None {
            if let Some(new_anim_handle) = player_state
                .get_state_from_texture_handle(AnimationType::DoubleJump, &resource_manager)
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if let Some(new_anim_handle) =
            player_state.get_state_from_texture_handle(AnimationType::Idle, &resource_manager)
        {
            *current_texture_atlas_handle = new_anim_handle;
        }
    });
}
