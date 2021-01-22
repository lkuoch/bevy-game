use crate::{
    player::{components::*, events::*, state::*, vars::*},
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
    for player in player_common::PLAYER_LIST.iter() {
        for texture in player_common::TEXTURES.iter() {
            let path = format!("{}{}", player.root_path.to_owned(), texture.path.to_owned());

            let texture_handle: Handle<Texture> = asset_server.load(&path[..]);
            let handle = texture_atlases.add(TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(32.0, 32.0),
                texture.frames,
                1,
            ));

            match player.ty {
                PlayerType::MaskDude => {
                    player_state.textures.insert(
                        PlayerSpriteMapKey::State(PlayerTypeKey::MaskDude(texture.state)),
                        PlayerSpriteKV::Handle(handle.clone()),
                    );

                    player_state.textures.insert(
                        PlayerSpriteKV::Handle(handle.clone()),
                        PlayerSpriteKV::State(texture.state),
                    );
                }
                PlayerType::NinjaFrog => {
                    player_state.textures.insert(
                        PlayerSpriteMapKey::State(PlayerTypeKey::NinjaFrog(texture.state)),
                        PlayerSpriteKV::Handle(handle.clone()),
                    );

                    player_state.textures.insert(
                        PlayerSpriteKV::Handle(handle.clone()),
                        PlayerSpriteKV::State(texture.state),
                    );
                }
                PlayerType::PinkMan => {
                    player_state.textures.insert(
                        PlayerSpriteMapKey::State(PlayerTypeKey::PinkMan(texture.state)),
                        PlayerSpriteKV::Handle(handle.clone()),
                    );

                    player_state.textures.insert(
                        PlayerSpriteKV::Handle(handle.clone()),
                        PlayerSpriteKV::State(texture.state),
                    );
                }
                PlayerType::VirtualGuy => {
                    player_state.textures.insert(
                        PlayerSpriteMapKey::State(PlayerTypeKey::VirtualGuy(texture.state)),
                        PlayerSpriteKV::Handle(handle.clone()),
                    );

                    player_state.textures.insert(
                        PlayerSpriteKV::Handle(handle.clone()),
                        PlayerSpriteKV::State(texture.state),
                    );
                }
            }
        }
    }

    // Default player is MaskDude
    if let Some(PlayerSpriteKV::Handle(default_texture)) =
        player_state
            .textures
            .get(&PlayerSpriteKV::State(PlayerTypeKey::MaskDude(
                player_common::States::Idle,
            )))
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
    mut events: ResMut<Events<AnimEvent>>,
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
                events.send(AnimEvent {
                    anim_start: player_state
                        .get_texture_handle_from_state(texture_atlas_handle.clone()),
                    anim_finish: None,
                });
            }

            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;

            // Anim finish
            if sprite.index as usize == texture_atlas.textures.len() - 1 {
                events.send(AnimEvent {
                    anim_start: None,
                    anim_finish: player_state
                        .get_texture_handle_from_state(texture_atlas_handle.clone()),
                });
            }
        }
    }
}

pub(super) fn handle_input_event(
    mut event_reader: EventReader<InputEvent>,
    mut player_state: ResMut<PlayerState>,
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

        // if let Some(key) = event.

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
    mut events: EventReader<AnimEvent>,
    mut player_state: ResMut<PlayerState>,
) {
    for event in events.iter() {
        if let Some(anim) = event.anim_finish {
            match anim {
                player_common::States::DoubleJump => player_state.land(),
                player_common::States::Idle => {}
                player_common::States::Fall => {}
                player_common::States::Hit => {}
                player_common::States::Jump => {}
                player_common::States::Run => {}
                player_common::States::WallJump => {}
            }
        }
    }
}

pub(super) fn react_player_state(
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform, &mut Handle<TextureAtlas>)>,
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

            transform.translation.x += time.delta_seconds() * dir_val * player_common::BASE_SPEED;
        }

        // Transformation
        if player_state.previous_sprite != player_state.current_sprite {
            match player_state.current_sprite {
                PlayerType::MaskDude => {
                    if let Some(PlayerSpriteKV::Handle(mask_dude_texture_handle)) = player_state
                        .textures
                        .get(&PlayerSpriteKV::State(PlayerTypeKey::MaskDude(
                            player_common::States::Idle,
                        )))
                    {
                        *current_texture_handle = mask_dude_texture_handle.clone();
                    }
                }
                PlayerType::NinjaFrog => {
                    if let Some(PlayerSpriteKV::Handle(ninja_frog_texture_handle)) = player_state
                        .textures
                        .get(&PlayerSpriteKV::State(PlayerTypeKey::NinjaFrog(
                            player_common::States::Idle,
                        )))
                    {
                        *current_texture_handle = ninja_frog_texture_handle.clone();
                    }
                }
                PlayerType::PinkMan => {
                    if let Some(PlayerSpriteKV::Handle(pink_man_texture_handle)) = player_state
                        .textures
                        .get(&PlayerSpriteKV::State(PlayerTypeKey::PinkMan(
                            player_common::States::Idle,
                        )))
                    {
                        *current_texture_handle = pink_man_texture_handle.clone();
                    }
                }
                PlayerType::VirtualGuy => {
                    if let Some(PlayerSpriteKV::Handle(virtual_guy_texture_handle)) =
                        player_state.textures.get(&PlayerSpriteKV::State(
                            PlayerTypeKey::VirtualGuy(player_common::States::Idle),
                        ))
                    {
                        *current_texture_handle = virtual_guy_texture_handle.clone();
                    }
                }
            }

            player_state.previous_sprite = player_state.current_sprite.clone();
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
                player_state.get_state_from_texture_handle(player_common::States::Run)
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if player_state.jump != JumpState::None {
            if let Some(new_anim_handle) =
                player_state.get_state_from_texture_handle(player_common::States::DoubleJump)
            {
                *current_texture_atlas_handle = new_anim_handle;
            }
        } else if let Some(new_anim_handle) =
            player_state.get_state_from_texture_handle(player_common::States::Idle)
        {
            *current_texture_atlas_handle = new_anim_handle;
        }
    }
}
