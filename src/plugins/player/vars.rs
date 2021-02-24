pub(super) mod player {
    use crate::plugins::player::components::PlayerType;

    pub const BASE_SPEED: f32 = 250.0;

    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub enum States {
        Idle,
        DoubleJump,
        Fall,
        Hit,
        Jump,
        Run,
        WallJump,
    }

    pub struct AnimationState<'a> {
        pub state: States,
        pub frames: usize,
        pub path: &'a str,
    }

    pub struct PlayerList<'a> {
        pub ty: PlayerType,
        pub root_path: &'a str,
    }

    pub const PLAYER_LIST: &[PlayerList] = &[
        PlayerList {
            ty: PlayerType::MaskDude,
            root_path: "pixels/Players/Mask Dude/",
        },
        PlayerList {
            ty: PlayerType::NinjaFrog,
            root_path: "pixels/Players/Ninja Frog/",
        },
        PlayerList {
            ty: PlayerType::PinkMan,
            root_path: "pixels/Players/Pink Man/",
        },
        PlayerList {
            ty: PlayerType::VirtualGuy,
            root_path: "pixels/Players/Virtual Guy/",
        },
    ];

    pub const TEXTURES: &[AnimationState] = &[
        AnimationState {
            state: States::Idle,
            frames: 11,
            path: "/Idle (32x32).png",
        },
        AnimationState {
            state: States::DoubleJump,
            frames: 6,
            path: "/Double Jump (32x32).png",
        },
        AnimationState {
            state: States::Fall,
            frames: 1,
            path: "/Fall (32x32).png",
        },
        AnimationState {
            state: States::Hit,
            frames: 7,
            path: "/Hit (32x32).png",
        },
        AnimationState {
            state: States::Jump,
            frames: 1,
            path: "/Jump (32x32).png",
        },
        AnimationState {
            state: States::Run,
            frames: 12,
            path: "/Run (32x32).png",
        },
        AnimationState {
            state: States::WallJump,
            frames: 5,
            path: "/Wall Jump (32x32).png",
        },
    ];
}
