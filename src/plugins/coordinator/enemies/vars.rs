pub mod enemies {
    use crate::coordinator::enemies::components::EnemyType;

    pub const BASE_SPEED: f32 = 200.0;

    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub enum States {
        Idle,
        DoubleJump,
        Fall,
        Hit,
        Hit2,
        Jump,
        Run,
        WallJump,
        Walk,
    }

    pub struct AnimationState<'a> {
        pub state: States,
        pub frames: usize,
        pub path: &'a str,
    }

    pub struct EnemyList<'a> {
        pub ty: EnemyType,
        pub animation_states: &'a [AnimationState<'a>],
        pub root_path: &'a str,
    }

    pub const ENEMY_LIST: &[EnemyList] = &[EnemyList {
        ty: EnemyType::AngryPig,

        animation_states: &[
            AnimationState {
                state: States::Idle,
                frames: 9,
                path: "/Idle (36x30).png",
            },
            AnimationState {
                state: States::Run,
                frames: 5,
                path: "Hit 2 (36x30).png",
            },
            AnimationState {
                state: States::Walk,
                frames: 12,
                path: "Run (36x30).png",
            },
            AnimationState {
                state: States::Hit,
                frames: 5,
                path: "Hit 1 (36x30).png",
            },
            AnimationState {
                state: States::Hit2,
                frames: 5,
                path: "Hit 2 (36x30).png",
            },
        ],
        root_path: "pixels/Enemies/AngryPig",
    }];
}
