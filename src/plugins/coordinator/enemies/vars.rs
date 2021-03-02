pub mod enemies {
    use crate::coordinator::enemies::components::*;
    use crate::plugins::core::components::*;

    pub const BASE_SPEED: f32 = 200.0;

    pub const ENEMY_LIST: &[EntList<AnimationType, EnemyType>] = &[EntList {
        animation_states: &[
            // ANGRY PIG
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    anim_ty: AnimationType::Idle,
                },
                frames: 9,
                tile_size: (32.0, 30.0),
                path: "AngryPig/Idle (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    anim_ty: AnimationType::Run,
                },
                frames: 5,
                tile_size: (36.0, 30.0),
                path: "AngryPig/Hit 2 (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    anim_ty: AnimationType::Walk,
                },
                frames: 12,
                tile_size: (36.0, 30.0),
                path: "AngryPig/Run (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (36.0, 30.0),
                path: "AngryPig/Hit 1 (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::AngryPig,
                    anim_ty: AnimationType::Hit2,
                },
                frames: 5,
                tile_size: (36.0, 30.0),
                path: "AngryPig/Hit 2 (36x30).png",
            },
            // BAT
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    anim_ty: AnimationType::CeilingIn,
                },
                frames: 7,
                tile_size: (46.0, 30.0),
                path: "Bat/Ceiling In (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    anim_ty: AnimationType::CeilingOut,
                },
                frames: 7,
                tile_size: (46.0, 30.0),
                path: "Bat/Ceiling Out (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    anim_ty: AnimationType::Flying,
                },
                frames: 7,
                tile_size: (46.0, 30.0),
                path: "Bat/Flying (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (46.0, 30.0),
                path: "Bat/Hit (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bat,
                    anim_ty: AnimationType::Idle,
                },
                frames: 12,
                tile_size: (46.0, 30.0),
                path: "Bat/Idle (46x30).png",
            },
            // BEE
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    anim_ty: AnimationType::Attack,
                },
                frames: 8,
                tile_size: (36.0, 34.0),
                path: "Bee/Attack (36x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    anim_ty: AnimationType::Bullet,
                },
                frames: 1,
                tile_size: (36.0, 34.0),
                path: "Bee/Bullet.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    anim_ty: AnimationType::BulletPieces,
                },
                frames: 2,
                tile_size: (32.0, 32.0),
                path: "Bee/Bullet Pieces.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (36.0, 34.0),
                path: "Bee/Hit (36x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bee,
                    anim_ty: AnimationType::Idle,
                },
                frames: 6,
                tile_size: (36.0, 34.0),
                path: "Bee/Idle (36x34).png",
            },
            // BLUE BIRD
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::BlueBird,
                    anim_ty: AnimationType::Flying,
                },
                frames: 9,
                tile_size: (32.0, 32.0),
                path: "BlueBird/Flying (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::BlueBird,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "BlueBird/Hit (32x32).png",
            },
            // BUNNY
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    anim_ty: AnimationType::Fall,
                },
                frames: 1,
                tile_size: (34.0, 44.0),
                path: "Bunny/Fall.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (34.0, 44.0),
                path: "Bunny/Hit (34x44).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    anim_ty: AnimationType::Idle,
                },
                frames: 8,
                tile_size: (34.0, 44.0),
                path: "Bunny/Idle (34x44).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    anim_ty: AnimationType::Jump,
                },
                frames: 1,
                tile_size: (34.0, 44.0),
                path: "Bunny/Jump.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Bunny,
                    anim_ty: AnimationType::Run,
                },
                frames: 12,
                tile_size: (34.0, 44.0),
                path: "Bunny/Run (34x44).png",
            },
            // CHAMELEON
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chameleon,
                    anim_ty: AnimationType::Attack,
                },
                frames: 10,
                tile_size: (84.0, 38.0),
                path: "Chameleon/Attack (84x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chameleon,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (84.0, 38.0),
                path: "Chameleon/Hit (84x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chameleon,
                    anim_ty: AnimationType::Idle,
                },
                frames: 13,
                tile_size: (84.0, 38.0),
                path: "Chameleon/Idle (84x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chameleon,
                    anim_ty: AnimationType::Run,
                },
                frames: 8,
                tile_size: (84.0, 38.0),
                path: "Chameleon/Run (84x38).png",
            },
            // CHICKEN
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chicken,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (32.0, 34.0),
                path: "Chicken/Hit (32x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chicken,
                    anim_ty: AnimationType::Idle,
                },
                frames: 13,
                tile_size: (32.0, 34.0),
                path: "Chicken/Idle (32x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Chicken,
                    anim_ty: AnimationType::Run,
                },
                frames: 14,
                tile_size: (32.0, 34.0),
                path: "Chicken/Run (32x34).png",
            },
            // DUCK
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Duck,
                    anim_ty: AnimationType::Fall,
                },
                frames: 1,
                tile_size: (36.0, 36.0),
                path: "Duck/Fall (36x36).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Duck,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (36.0, 36.0),
                path: "Duck/Hit (36x36).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Duck,
                    anim_ty: AnimationType::Idle,
                },
                frames: 10,
                tile_size: (36.0, 36.0),
                path: "Duck/Idle (36x36).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Duck,
                    anim_ty: AnimationType::Jump,
                },
                frames: 1,
                tile_size: (36.0, 36.0),
                path: "Duck/Jump (36x36).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Duck,
                    anim_ty: AnimationType::JumpAnticipation,
                },
                frames: 4,
                tile_size: (36.0, 36.0),
                path: "Duck/Jump Anticipation (36x36).png",
            },
            // FAT BIRD
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::FatBird,
                    anim_ty: AnimationType::Fall,
                },
                frames: 4,
                tile_size: (40.0, 48.0),
                path: "FatBird/Fall (40x48).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::FatBird,
                    anim_ty: AnimationType::Ground,
                },
                frames: 4,
                tile_size: (40.0, 48.0),
                path: "FatBird/Ground (40x48).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::FatBird,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (40.0, 48.0),
                path: "FatBird/Hit (40x48).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::FatBird,
                    anim_ty: AnimationType::Idle,
                },
                frames: 8,
                tile_size: (40.0, 48.0),
                path: "FatBird/Idle (40x48).png",
            },
            // GHOST
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Ghost,
                    anim_ty: AnimationType::Appear,
                },
                frames: 4,
                tile_size: (44.0, 30.0),
                path: "Ghost/Appear (44x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Ghost,
                    anim_ty: AnimationType::Disappear,
                },
                frames: 5,
                tile_size: (44.0, 30.0),
                path: "Ghost/Disappear (44x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Ghost,
                    anim_ty: AnimationType::GhostParticles,
                },
                frames: 4,
                tile_size: (48.0, 16.0),
                path: "Ghost/Ghost Particles (48x16).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Ghost,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (44.0, 30.0),
                path: "Ghost/Hit (44x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Ghost,
                    anim_ty: AnimationType::Idle,
                },
                frames: 10,
                tile_size: (44.0, 30.0),
                path: "Ghost/Idle (44x30).png",
            },
            // MUSHROOM
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Mushroom,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Mushroom/Hit.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Mushroom,
                    anim_ty: AnimationType::Idle,
                },
                frames: 14,
                tile_size: (32.0, 32.0),
                path: "Mushroom/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Mushroom,
                    anim_ty: AnimationType::Run,
                },
                frames: 16,
                tile_size: (32.0, 32.0),
                path: "Mushroom/Run (32x32).png",
            },
            // PLANT
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Plant,
                    anim_ty: AnimationType::Attack,
                },
                frames: 8,
                tile_size: (42.0, 42.0),
                path: "Plant/Attack (44x42).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Plant,
                    anim_ty: AnimationType::BulletPieces,
                },
                frames: 2,
                tile_size: (32.0, 32.0),
                path: "Plant/Bullet Pieces.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Plant,
                    anim_ty: AnimationType::Bullet,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Plant/Bullet.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Plant,
                    anim_ty: AnimationType::Hit,
                },
                frames: 5,
                tile_size: (44.0, 42.0),
                path: "Plant/Hit (44x42).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyType::Plant,
                    anim_ty: AnimationType::Idle,
                },
                frames: 11,
                tile_size: (44.0, 42.0),
                path: "Plant/Idle (44x42).png",
            },
        ],
        root_path: "pixels/Enemies/",
    }];
}
