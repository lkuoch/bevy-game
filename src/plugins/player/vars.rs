pub mod player {
    use crate::plugins::{animation::components::*, player::components::*};

    pub const BASE_SPEED: f32 = 250.0;

    pub const PLAYER_ANIMATIONS: &[EntList<PlayerAnimationStates, PlayerTypeStates>] = &[EntList {
        animation_states: &[
            // MASK DUDE
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Idle,
                    ty: PlayerTypeStates::MaskDude,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::DoubleJump,
                    ty: PlayerTypeStates::MaskDude,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Fall,
                    ty: PlayerTypeStates::MaskDude,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Hit,
                    ty: PlayerTypeStates::MaskDude,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::MaskDude,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::MaskDude,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Run,
                    ty: PlayerTypeStates::MaskDude,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::WallJump,
                    ty: PlayerTypeStates::MaskDude,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Wall Jump (32x32).png",
            },
            // NINJA FROG
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Idle,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::DoubleJump,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Fall,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Hit,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Run,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::WallJump,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Wall Jump (32x32).png",
            },
            // NINJA FROG
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Idle,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::DoubleJump,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Fall,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Hit,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Run,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::WallJump,
                    ty: PlayerTypeStates::NinjaFrog,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Wall Jump (32x32).png",
            },
            // PINK MAN
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Idle,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::DoubleJump,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Fall,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Hit,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Run,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::WallJump,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Wall Jump (32x32).png",
            },
            // Pink Man
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Idle,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::DoubleJump,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Fall,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Hit,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Run,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::WallJump,
                    ty: PlayerTypeStates::PinkMan,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Wall Jump (32x32).png",
            },
            // VIRTUAL GUY
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Idle,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::DoubleJump,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Fall,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Hit,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Run,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::WallJump,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Wall Jump (32x32).png",
            },
            // Virtual Guy
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Idle,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::DoubleJump,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Fall,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Hit,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Jump,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::Run,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: PlayerAnimationStates::WallJump,
                    ty: PlayerTypeStates::VirtualGuy,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Wall Jump (32x32).png",
            },
        ],
        root_path: "pixels/Players/",
    }];
}
