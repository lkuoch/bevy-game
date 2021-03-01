use crate::{
    player::{components::*, vars::*},
    plugins::{
        core::{components::*, traits::*},
        input::components::*,
    },
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut player_state: ResMut<Player>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for player in player::PLAYER_LIST.iter() {
        for anim in player.animation_states.iter() {
            let path = format!("{}{}", player.root_path.to_owned(), anim.path.to_owned());

            let texture_handle: Handle<Texture> = asset_server.load(&path[..]);
            let handle = texture_atlases.add(TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(anim.tile_size.0, anim.tile_size.1),
                anim.frames,
                1,
            ));

            player_state.textures.insert(
                PlayerSpriteMapKey::State(anim.kv),
                EntSpriteKV::Handle(handle.clone()),
            );

            player_state.textures.insert(
                EntSpriteKV::Handle(handle.clone()),
                EntSpriteKV::State(anim.kv.state),
            );
        }
    }

    // Default player is MaskDude
    if let Some(EntSpriteKV::Handle(default_texture)) = player_state
        .textures
        .get(&EntSpriteKV::State(player::DEFAULT_PLAYER))
    {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: default_texture.clone(),
                transform: Transform::from_scale(Vec3::splat(2.5)),
                ..Default::default()
            })
            .with(PlayerTag)
            .with(AnimatableTag)
            .with(Timer::from_seconds(0.1, true));
    }
}

pub fn handle_input_event(
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

pub fn handle_animation(
    mut events: EventReader<AnimEvent<Handle<TextureAtlas>>>,
    mut player_state: ResMut<Player>,
) {
    for event in events.iter() {
        match event {
            AnimEvent::Start(_handle) => {}
            AnimEvent::Finish(handle) => {
                if let Some(state) = player_state.get_texture_handle_from_state(handle.clone(), ())
                {
                    match state {
                        States::DoubleJump => player_state.land(),
                        States::Idle => {}
                        States::Fall => {}
                        States::Hit => {}
                        States::Jump => {}
                        States::Run => {}
                        States::WallJump => {}
                    }
                }
            }
            AnimEvent::None => {}
        }
    }
}

pub fn observe_player_state(
    mut player_state: ResMut<Player>,
    time: Res<Time>,
    mut query: Query<(&PlayerTag, &mut Transform, &mut Handle<TextureAtlas>)>,
) {
    for (_, mut transform, mut current_texture_handle) in query.iter_mut() {
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
            if let Some(EntSpriteKV::Handle(mask_dude_texture_handle)) =
                player_state.textures.get(&EntSpriteKV::State(EntTypeKey {
                    ty: player_state.current_sprite,
                    state: States::Idle,
                }))
            {
                *current_texture_handle = mask_dude_texture_handle.clone();
            }

            player_state.previous_sprite = player_state.current_sprite.clone();
        }
    }
}

pub fn change_animation(
    player_state: Res<Player>,
    mut query: Query<(&PlayerTag, &mut Handle<TextureAtlas>)>,
) {
    for (_, mut current_texture_atlas_handle) in query.iter_mut() {
        if player_state.movement != MovementState::None {
            if let Some(new_anim_handle) =
                player_state.get_state_from_texture_handle(States::Run, ())
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if player_state.jump != JumpState::None {
            if let Some(new_anim_handle) =
                player_state.get_state_from_texture_handle(States::DoubleJump, ())
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if let Some(new_anim_handle) =
            player_state.get_state_from_texture_handle(States::Idle, ())
        {
            *current_texture_atlas_handle = new_anim_handle;
        }
    }
}
