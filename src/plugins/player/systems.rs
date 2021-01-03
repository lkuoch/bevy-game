use crate::{
    player::{components::*, events::PlayerEvent, state::*, vars::mask_dude as MaskDude},
    plugins::input::events::InputEvent,
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub(super) fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut player_state: ResMut<PlayerState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load MaskDude textures
    for texture in MaskDude::TEXTURES {
        let texture_handle = asset_server.load(texture.path);

        let handle = texture_atlases.add(TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(32.0, 32.0),
            texture.frames,
            1,
        ));

        player_state.mask_dude_textures.insert(
            MaskDudeTextureKeyValue::State(texture.state),
            MaskDudeTextureKeyValue::Handle(handle.clone()),
        );

        player_state.mask_dude_textures.insert(
            MaskDudeTextureKeyValue::Handle(handle.clone()),
            MaskDudeTextureKeyValue::State(texture.state),
        );
    }

    if let Some(MaskDudeTextureKeyValue::Handle(default_texture)) = player_state
        .mask_dude_textures
        .get(&MaskDudeTextureKeyValue::State(MaskDude::States::Idle))
    {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: default_texture.clone(),
                transform: Transform::from_scale(Vec3::splat(2.5)),
                ..Default::default()
            })
            .with(Player)
            .with(Timer::from_seconds(0.1, true));
    }
}

pub(super) fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    player_state: Res<PlayerState>,
    mut events: ResMut<Events<PlayerEvent>>,
    mut query: Query<(
        &Player,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (_, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            // Anim start
            if sprite.index == 0 {
                events.send(PlayerEvent {
                    anim_start: player_state
                        .get_texture_handle_from_mask_dude_state(texture_atlas_handle.clone()),
                    anim_finish: None,
                });
            }

            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;

            // Anim finish
            if sprite.index as usize == texture_atlas.textures.len() - 1 {
                events.send(PlayerEvent {
                    anim_start: None,
                    anim_finish: player_state
                        .get_texture_handle_from_mask_dude_state(texture_atlas_handle.clone()),
                });
            }
        }
    }
}

pub(super) fn handle_input_event(
    mut event_reader: Local<EventReader<InputEvent>>,
    events: ResMut<Events<InputEvent>>,
    mut player_state: ResMut<PlayerState>,
) {
    for event in event_reader.iter(&events) {
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

pub(super) fn handle_player_event(
    mut event_reader: Local<EventReader<PlayerEvent>>,
    events: ResMut<Events<PlayerEvent>>,
    mut player_state: ResMut<PlayerState>,
) {
    for event in event_reader.iter(&events) {
        if let Some(anim) = event.anim_finish {
            match anim {
                MaskDude::States::DoubleJump => player_state.land(),
                MaskDude::States::Idle => {}
                MaskDude::States::Fall => {}
                MaskDude::States::Hit => {}
                MaskDude::States::Jump => {}
                MaskDude::States::Run => {}
                MaskDude::States::WallJump => {}
            }
        }
    }
}

pub(super) fn react_player_state(
    player_state: Res<PlayerState>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
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

            transform.translation.x += time.delta_seconds() * dir_val * MaskDude::BASE_SPEED;
        }
    }
}

pub(super) fn change_animation(
    player_state: Res<PlayerState>,
    mut query: Query<(&Player, &mut Handle<TextureAtlas>)>,
) {
    for (_, mut current_texture_atlas_handle) in query.iter_mut() {
        if player_state.movement != MovementState::None {
            if let Some(new_anim_handle) =
                player_state.get_mask_dude_state_from_texture_handle(MaskDude::States::Run)
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if player_state.jump != JumpState::None {
            if let Some(new_anim_handle) =
                player_state.get_mask_dude_state_from_texture_handle(MaskDude::States::DoubleJump)
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if let Some(new_anim_handle) =
            player_state.get_mask_dude_state_from_texture_handle(MaskDude::States::Idle)
        {
            *current_texture_atlas_handle = new_anim_handle;
        }
    }
}
