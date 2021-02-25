pub mod enemies {
    use crate::coordinator::enemies::components::*;

    pub const BASE_SPEED: f32 = 200.0;

    pub const ENEMY_LIST: &[EnemyList<States>] = &[EnemyList {
        animation_states: &[
            // ANGRY PIG
            AnimationState {
                kv: EnemyTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Idle,
                },
                frames: 9,
                path: "Idle (36x30).png",
            },
            AnimationState {
                kv: EnemyTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Run,
                },
                frames: 5,
                path: "Hit 2 (36x30).png",
            },
            AnimationState {
                kv: EnemyTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Walk,
                },
                frames: 12,
                path: "Run (36x30).png",
            },
            AnimationState {
                kv: EnemyTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Hit,
                },
                frames: 5,
                path: "Hit 1 (36x30).png",
            },
            AnimationState {
                kv: EnemyTypeKey {
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
