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
    mut player: ResMut<Player>,
) {
    for event in event_reader.iter() {
        if let Some(key) = event.pressed {
            match key {
                KeyCode::A => {
                    player
                        .state
                        .movement
                        .enqueue(PlayerCommand::Movement(PlayerMovementDirection::Left));
                }
                KeyCode::D => {
                    player
                        .state
                        .movement
                        .enqueue(PlayerCommand::Movement(PlayerMovementDirection::Right));
                }
                KeyCode::Space => {
                    player.state.movement.enqueue(PlayerCommand::Jump);
                }
                KeyCode::T => player.state.movement.enqueue(PlayerCommand::Transform),
                _ => {}
            }
        }

        if let Some(key) = event.released {
            match key {
                KeyCode::A => {
                    player
                        .state
                        .movement
                        .enqueue(PlayerCommand::MovementComplete);
                }
                KeyCode::D => {
                    player
                        .state
                        .movement
                        .enqueue(PlayerCommand::MovementComplete);
                }
                _ => {}
            }
        }
    }
}

// TODO: Incorporate new state
pub fn handle_animation_system(
    resource_manager: Res<ResourceManager>,
    mut events: EventReader<AnimEvent<Handle<TextureAtlas>>>,
    mut player: ResMut<Player>,
) {
    for event in events.iter() {
        match event {
            AnimEvent::Start(_handle) => {}
            AnimEvent::Finish(handle) => {
                if let Some(state) =
                    player.get_texture_handle_from_state(handle.clone(), &resource_manager)
                {
                    match state {
                        AnimationType::DoubleJump => {}
                        AnimationType::Idle => {}
                        AnimationType::Fall => {}
                        AnimationType::Hit => {}
                        AnimationType::Jump => {}
                        AnimationType::Run => {}
                        AnimationType::WallJump => {}
                    }
                }
            }
        }
    }
}

// TODO: Figure out how to do this in stages
pub fn observe_player_state_system(
    time: Res<Time>,
    resource_manager: Res<ResourceManager>,
    mut player: ResMut<Player>,
    mut query: Query<(&PlayerTag, &mut Transform, &mut Handle<TextureAtlas>)>,
) {
    query.for_each_mut(|(_, mut transform, mut current_texture_handle)| {
        // Movement
        if let MovementState::Moving(direction) = player.state.movement.get() {
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

        // Transformation
        if player.previous_sprite != player.current_sprite {
            if let Some(EntSpriteKV::Handle(mask_dude_texture_handle)) = resource_manager
                .textures
                .player
                .get(&EntSpriteKV::State(EntTypeKey {
                    ty: player.current_sprite,
                    anim_ty: AnimationType::Idle,
                }))
            {
                *current_texture_handle = mask_dude_texture_handle.clone();
            }

            player.previous_sprite = player.current_sprite.clone();
        }
    });
}

// TODO: Figure out how to make this better
pub fn change_animation_system(
    player: Res<Player>,
    resource_manager: Res<ResourceManager>,
    mut query: Query<(&PlayerTag, &mut Handle<TextureAtlas>)>,
) {
    query.for_each_mut(|(_, mut current_texture_atlas_handle)| {
        if player.state.movement.get() == MovementState::Moving(PlayerMovementDirection::Left)
            || player.state.movement.get() == MovementState::Moving(PlayerMovementDirection::Right)
        {
            if let Some(new_anim_handle) =
                player.get_state_from_texture_handle(AnimationType::Run, &resource_manager)
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if player.state.movement.get() == MovementState::Jumping {
            if let Some(new_anim_handle) =
                player.get_state_from_texture_handle(AnimationType::DoubleJump, &resource_manager)
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if let Some(new_anim_handle) =
            player.get_state_from_texture_handle(AnimationType::Idle, &resource_manager)
        {
            *current_texture_atlas_handle = new_anim_handle;
        }
    });
}
