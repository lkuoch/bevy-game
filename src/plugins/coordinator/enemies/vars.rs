pub mod enemies {
    use crate::coordinator::enemies::components::*;
    use crate::plugins::core::components::*;

    pub const BASE_SPEED: f32 = 200.0;

    pub const DEFAULT_ANGRY_PIG: EntTypeKey<States, EnemyType> = EntTypeKey {
        ty: EnemyType::AngryPig,
        state: States::Idle,
    };

    pub const ENEMY_LIST: &[EntList<States, EnemyType>] = &[EntList {
        animation_states: &[
            // ANGRY PIG
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Idle,
                },
                frames: 9,
                path: "Idle (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Run,
                },
                frames: 5,
                path: "Hit 2 (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Walk,
                },
                frames: 12,
                path: "Run (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Hit,
                },
                frames: 5,
                path: "Hit 1 (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Hit2,
                },
                frames: 5,
                path: "Hit 2 (36x30).png",
            },
        ],
        root_path: "pixels/Enemies/AngryPig/",
    }];
}
