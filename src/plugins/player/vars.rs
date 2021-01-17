pub (super) mod player_common {
    pub const BASE_SPEED: f32 = 250.0;
}

pub(super) mod mask_dude {
    const ROOT_PATH: &str = "pixels/Players/Mask Dude/";

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

    pub const TEXTURES: &[AnimationState] = &[
        AnimationState {
            state: States::Idle,
            frames: 11,
            path: const_format::concatcp!(ROOT_PATH, "/Idle (32x32).png"),
        },
        AnimationState {
            state: States::DoubleJump,
            frames: 6,
            path: const_format::concatcp!(ROOT_PATH, "/Double Jump (32x32).png"),
        },
        AnimationState {
            state: States::Fall,
            frames: 1,
            path: const_format::concatcp!(ROOT_PATH, "/Fall (32x32).png"),
        },
        AnimationState {
            state: States::Hit,
            frames: 7,
            path: const_format::concatcp!(ROOT_PATH, "/Hit (32x32).png"),
        },
        AnimationState {
            state: States::Jump,
            frames: 1,
            path: const_format::concatcp!(ROOT_PATH, "/Jump (32x32).png"),
        },
        AnimationState {
            state: States::Run,
            frames: 12,
            path: const_format::concatcp!(ROOT_PATH, "/Run (32x32).png"),
        },
        AnimationState {
            state: States::WallJump,
            frames: 5,
            path: const_format::concatcp!(ROOT_PATH, "/Wall Jump (32x32).png"),
        },
    ];
}

pub(super) mod ninja_frog {
    const ROOT_PATH: &str = "pixels/Players/Ninja Frog/";

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

    pub const TEXTURES: &[AnimationState] = &[
        AnimationState {
            state: States::Idle,
            frames: 11,
            path: const_format::concatcp!(ROOT_PATH, "/Idle (32x32).png"),
        },
        AnimationState {
            state: States::DoubleJump,
            frames: 6,
            path: const_format::concatcp!(ROOT_PATH, "/Double Jump (32x32).png"),
        },
        AnimationState {
            state: States::Fall,
            frames: 1,
            path: const_format::concatcp!(ROOT_PATH, "/Fall (32x32).png"),
        },
        AnimationState {
            state: States::Hit,
            frames: 7,
            path: const_format::concatcp!(ROOT_PATH, "/Hit (32x32).png"),
        },
        AnimationState {
            state: States::Jump,
            frames: 1,
            path: const_format::concatcp!(ROOT_PATH, "/Jump (32x32).png"),
        },
        AnimationState {
            state: States::Run,
            frames: 12,
            path: const_format::concatcp!(ROOT_PATH, "/Run (32x32).png"),
        },
        AnimationState {
            state: States::WallJump,
            frames: 5,
            path: const_format::concatcp!(ROOT_PATH, "/Wall Jump (32x32).png"),
        },
    ];
}

pub(super) mod pink_man {
    const ROOT_PATH: &str = "pixels/Players/Pink Man/";

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

    pub const TEXTURES: &[AnimationState] = &[
        AnimationState {
            state: States::Idle,
            frames: 11,
            path: const_format::concatcp!(ROOT_PATH, "/Idle (32x32).png"),
        },
        AnimationState {
            state: States::DoubleJump,
            frames: 6,
            path: const_format::concatcp!(ROOT_PATH, "/Double Jump (32x32).png"),
        },
        AnimationState {
            state: States::Fall,
            frames: 1,
            path: const_format::concatcp!(ROOT_PATH, "/Fall (32x32).png"),
        },
        AnimationState {
            state: States::Hit,
            frames: 7,
            path: const_format::concatcp!(ROOT_PATH, "/Hit (32x32).png"),
        },
        AnimationState {
            state: States::Jump,
            frames: 1,
            path: const_format::concatcp!(ROOT_PATH, "/Jump (32x32).png"),
        },
        AnimationState {
            state: States::Run,
            frames: 12,
            path: const_format::concatcp!(ROOT_PATH, "/Run (32x32).png"),
        },
        AnimationState {
            state: States::WallJump,
            frames: 5,
            path: const_format::concatcp!(ROOT_PATH, "/Wall Jump (32x32).png"),
        },
    ];
}

pub(super) mod virtual_guy {
    const ROOT_PATH: &str = "pixels/Players/Virtual Guy/";

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

    pub const TEXTURES: &[AnimationState] = &[
        AnimationState {
            state: States::Idle,
            frames: 11,
            path: const_format::concatcp!(ROOT_PATH, "/Idle (32x32).png"),
        },
        AnimationState {
            state: States::DoubleJump,
            frames: 6,
            path: const_format::concatcp!(ROOT_PATH, "/Double Jump (32x32).png"),
        },
        AnimationState {
            state: States::Fall,
            frames: 1,
            path: const_format::concatcp!(ROOT_PATH, "/Fall (32x32).png"),
        },
        AnimationState {
            state: States::Hit,
            frames: 7,
            path: const_format::concatcp!(ROOT_PATH, "/Hit (32x32).png"),
        },
        AnimationState {
            state: States::Jump,
            frames: 1,
            path: const_format::concatcp!(ROOT_PATH, "/Jump (32x32).png"),
        },
        AnimationState {
            state: States::Run,
            frames: 12,
            path: const_format::concatcp!(ROOT_PATH, "/Run (32x32).png"),
        },
        AnimationState {
            state: States::WallJump,
            frames: 5,
            path: const_format::concatcp!(ROOT_PATH, "/Wall Jump (32x32).png"),
        },
    ];
}
