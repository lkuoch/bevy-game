pub mod player {
    use crate::plugins::{animation::components::*, player::components::*};

    pub const BASE_SPEED: f32 = 250.0;

    pub const PLAYER_ANIMATIONS: &[EntList<AnimationType, PlayerTransformationState>] = &[EntList {
        animation_states: &[
            // MASK DUDE
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerTransformationState::MaskDude,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerTransformationState::MaskDude,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerTransformationState::MaskDude,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerTransformationState::MaskDude,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::MaskDude,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::MaskDude,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerTransformationState::MaskDude,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerTransformationState::MaskDude,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Wall Jump (32x32).png",
            },
            // NINJA FROG
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Wall Jump (32x32).png",
            },
            // NINJA FROG
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerTransformationState::NinjaFrog,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Wall Jump (32x32).png",
            },
            // PINK MAN
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Wall Jump (32x32).png",
            },
            // Pink Man
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerTransformationState::PinkMan,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Wall Jump (32x32).png",
            },
            // VIRTUAL GUY
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Wall Jump (32x32).png",
            },
            // Virtual Guy
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerTransformationState::VirtualGuy,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Wall Jump (32x32).png",
            },
        ],
        root_path: "pixels/Players/",
    }];
}
