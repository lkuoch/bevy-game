pub(super) mod player {
    use crate::plugins::player::components::*;

    pub const BASE_SPEED: f32 = 250.0;

    pub const PLAYER_LIST: &[PlayerList<States>] = &[PlayerList {
        animation_states: &[
            // MASK DUDE
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Idle,
                    ty: PlayerType::MaskDude,
                },
                frames: 11,
                path: "Mask Dude/Idle (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::DoubleJump,
                    ty: PlayerType::MaskDude,
                },
                frames: 6,
                path: "Mask Dude/Double Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Fall,
                    ty: PlayerType::MaskDude,
                },
                frames: 1,
                path: "Mask Dude/Fall (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Hit,
                    ty: PlayerType::MaskDude,
                },
                frames: 7,
                path: "Mask Dude/Hit (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::MaskDude,
                },
                frames: 1,
                path: "Mask Dude/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::MaskDude,
                },
                frames: 1,
                path: "Mask Dude/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Run,
                    ty: PlayerType::MaskDude,
                },
                frames: 12,
                path: "Mask Dude/Run (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::WallJump,
                    ty: PlayerType::MaskDude,
                },
                frames: 5,
                path: "Mask Dude/Wall Jump (32x32).png",
            },
            // NINJA FROG
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Idle,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 11,
                path: "Ninja Frog/Idle (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::DoubleJump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 6,
                path: "Ninja Frog/Double Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Fall,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                path: "Ninja Frog/Fall (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Hit,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 7,
                path: "Ninja Frog/Hit (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Run,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 12,
                path: "Ninja Frog/Run (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::WallJump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 5,
                path: "Ninja Frog/Wall Jump (32x32).png",
            },
            // NINJA FROG
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Idle,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 11,
                path: "Ninja Frog/Idle (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::DoubleJump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 6,
                path: "Ninja Frog/Double Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Fall,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                path: "Ninja Frog/Fall (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Hit,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 7,
                path: "Ninja Frog/Hit (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Run,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 12,
                path: "Ninja Frog/Run (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::WallJump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 5,
                path: "Ninja Frog/Wall Jump (32x32).png",
            },
            // PINK MAN
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Idle,
                    ty: PlayerType::PinkMan,
                },
                frames: 11,
                path: "Pink Man/Idle (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::DoubleJump,
                    ty: PlayerType::PinkMan,
                },
                frames: 6,
                path: "Pink Man/Double Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Fall,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                path: "Pink Man/Fall (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Hit,
                    ty: PlayerType::PinkMan,
                },
                frames: 7,
                path: "Pink Man/Hit (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Run,
                    ty: PlayerType::PinkMan,
                },
                frames: 12,
                path: "Pink Man/Run (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::WallJump,
                    ty: PlayerType::PinkMan,
                },
                frames: 5,
                path: "Pink Man/Wall Jump (32x32).png",
            },
            // Pink Man
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Idle,
                    ty: PlayerType::PinkMan,
                },
                frames: 11,
                path: "Pink Man/Idle (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::DoubleJump,
                    ty: PlayerType::PinkMan,
                },
                frames: 6,
                path: "Pink Man/Double Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Fall,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                path: "Pink Man/Fall (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Hit,
                    ty: PlayerType::PinkMan,
                },
                frames: 7,
                path: "Pink Man/Hit (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Run,
                    ty: PlayerType::PinkMan,
                },
                frames: 12,
                path: "Pink Man/Run (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::WallJump,
                    ty: PlayerType::PinkMan,
                },
                frames: 5,
                path: "Pink Man/Wall Jump (32x32).png",
            },
            // VIRTUAL GUY
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Idle,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 11,
                path: "Virtual Guy/Idle (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::DoubleJump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 6,
                path: "Virtual Guy/Double Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Fall,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                path: "Virtual Guy/Fall (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Hit,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 7,
                path: "Virtual Guy/Hit (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Run,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 12,
                path: "Virtual Guy/Run (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::WallJump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 5,
                path: "Virtual Guy/Wall Jump (32x32).png",
            },
            // Virtual Guy
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Idle,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 11,
                path: "Virtual Guy/Idle (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::DoubleJump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 6,
                path: "Virtual Guy/Double Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Fall,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                path: "Virtual Guy/Fall (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Hit,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 7,
                path: "Virtual Guy/Hit (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Jump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::Run,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 12,
                path: "Virtual Guy/Run (32x32).png",
            },
            AnimationState {
                kv: PlayerTypeKey {
                    state: States::WallJump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 5,
                path: "Virtual Guy/Wall Jump (32x32).png",
            },
        ],
        root_path: "pixels/Players/",
    }];
}
