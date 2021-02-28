pub mod enemies {
    use crate::coordinator::enemies::components::*;
    use crate::plugins::core::components::*;

    pub const BASE_SPEED: f32 = 200.0;

    pub const ENEMY_LIST: &[EntList<States, EnemyType>] = &[EntList {
        animation_states: &[
            // ANGRY PIG
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Idle,
                },
                frames: 9,
                path: "AngryPig/Idle (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Run,
                },
                frames: 5,
                path: "AngryPig/Hit 2 (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Walk,
                },
                frames: 12,
                path: "AngryPig/Run (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Hit,
                },
                frames: 5,
                path: "AngryPig/Hit 1 (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    state: States::Hit2,
                },
                frames: 5,
                path: "AngryPig/Hit 2 (36x30).png",
            },
            // BAT
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    state: States::CeilingIn,
                },
                frames: 7,
                path: "Bat/Ceiling In (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    state: States::CeilingOut,
                },
                frames: 7,
                path: "Bat/Ceiling Out (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    state: States::Flying,
                },
                frames: 7,
                path: "Bat/Flying (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    state: States::Hit,
                },
                frames: 5,
                path: "Bat/Hit (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    state: States::Idle,
                },
                frames: 12,
                path: "Bat/Idle (46x30).png",
            },
            // BEE
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    state: States::Attack,
                },
                frames: 8,
                path: "Bee/Attack (36x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    state: States::Bullet,
                },
                frames: 1,
                path: "Bee/Bullet.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    state: States::BulletPieces,
                },
                frames: 2,
                path: "Bee/Bullet Pieces.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    state: States::Hit,
                },
                frames: 5,
                path: "Bee/Hit (36x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    state: States::Idle,
                },
                frames: 6,
                path: "Bee/Idle (36x34).png",
            },
            // BLUE BIRD
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::BlueBird,
                    state: States::Flying,
                },
                frames: 9,
                path: "BlueBird/Flying (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::BlueBird,
                    state: States::Hit,
                },
                frames: 5,
                path: "BlueBird/Hit (32x32).png",
            },
            // BUNNY
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    state: States::Fall,
                },
                frames: 1,
                path: "Bunny/Fall.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    state: States::Hit,
                },
                frames: 5,
                path: "Bunny/Hit (34x44).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    state: States::Idle,
                },
                frames: 8,
                path: "Bunny/Idle (34x44).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    state: States::Jump,
                },
                frames: 1,
                path: "Bunny/Jump.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    state: States::Run,
                },
                frames: 12,
                path: "Bunny/Run (34x44).png",
            },
            // CHAMELEON
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chameleon,
                    state: States::Attack,
                },
                frames: 10,
                path: "Chameleon/Attack (84x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chameleon,
                    state: States::Hit,
                },
                frames: 5,
                path: "Chameleon/Hit (84x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chameleon,
                    state: States::Idle,
                },
                frames: 13,
                path: "Chameleon/Idle (84x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chameleon,
                    state: States::Run,
                },
                frames: 8,
                path: "Chameleon/Run (84x38).png",
            },
        ],
        root_path: "pixels/Enemies/",
    }];
}


