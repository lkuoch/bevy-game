pub mod enemies {
    use crate::{coordinator::enemies::components::*, plugins::animation::components::*};

    // pub const BASE_SPEED: f32 = 200.0;

    // DO THIS WITHOUT HARDCODING :(
    pub const ENEMY_ANIMATIONS: &[EntList<EnemyAnimationStates, EnemyTypeStates>] = &[EntList {
        animation_states: &[
            // ANGRY PIG
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::AngryPig,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 9,
                tile_size: (32.0, 30.0),
                path: "AngryPig/Idle (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::AngryPig,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 5,
                tile_size: (36.0, 30.0),
                path: "AngryPig/Hit 2 (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::AngryPig,
                    anim_ty: EnemyAnimationStates::Walk,
                },
                frames: 12,
                tile_size: (36.0, 30.0),
                path: "AngryPig/Run (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::AngryPig,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (36.0, 30.0),
                path: "AngryPig/Hit 1 (36x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::AngryPig,
                    anim_ty: EnemyAnimationStates::Hit2,
                },
                frames: 5,
                tile_size: (36.0, 30.0),
                path: "AngryPig/Hit 2 (36x30).png",
            },
            // BAT
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bat,
                    anim_ty: EnemyAnimationStates::CeilingIn,
                },
                frames: 7,
                tile_size: (46.0, 30.0),
                path: "Bat/Ceiling In (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bat,
                    anim_ty: EnemyAnimationStates::CeilingOut,
                },
                frames: 7,
                tile_size: (46.0, 30.0),
                path: "Bat/Ceiling Out (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bat,
                    anim_ty: EnemyAnimationStates::Flying,
                },
                frames: 7,
                tile_size: (46.0, 30.0),
                path: "Bat/Flying (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bat,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (46.0, 30.0),
                path: "Bat/Hit (46x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bat,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 12,
                tile_size: (46.0, 30.0),
                path: "Bat/Idle (46x30).png",
            },
            // BEE
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bee,
                    anim_ty: EnemyAnimationStates::Attack,
                },
                frames: 8,
                tile_size: (36.0, 34.0),
                path: "Bee/Attack (36x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bee,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (36.0, 34.0),
                path: "Bee/Hit (36x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bee,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 6,
                tile_size: (36.0, 34.0),
                path: "Bee/Idle (36x34).png",
            },
            // BLUE BIRD
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::BlueBird,
                    anim_ty: EnemyAnimationStates::Flying,
                },
                frames: 9,
                tile_size: (32.0, 32.0),
                path: "BlueBird/Flying (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::BlueBird,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "BlueBird/Hit (32x32).png",
            },
            // BUNNY
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bunny,
                    anim_ty: EnemyAnimationStates::Fall,
                },
                frames: 1,
                tile_size: (34.0, 44.0),
                path: "Bunny/Fall.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bunny,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (34.0, 44.0),
                path: "Bunny/Hit (34x44).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bunny,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 8,
                tile_size: (34.0, 44.0),
                path: "Bunny/Idle (34x44).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bunny,
                    anim_ty: EnemyAnimationStates::Jump,
                },
                frames: 1,
                tile_size: (34.0, 44.0),
                path: "Bunny/Jump.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Bunny,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 12,
                tile_size: (34.0, 44.0),
                path: "Bunny/Run (34x44).png",
            },
            // CHAMELEON
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Chameleon,
                    anim_ty: EnemyAnimationStates::Attack,
                },
                frames: 10,
                tile_size: (84.0, 38.0),
                path: "Chameleon/Attack (84x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Chameleon,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (84.0, 38.0),
                path: "Chameleon/Hit (84x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Chameleon,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 13,
                tile_size: (84.0, 38.0),
                path: "Chameleon/Idle (84x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Chameleon,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 8,
                tile_size: (84.0, 38.0),
                path: "Chameleon/Run (84x38).png",
            },
            // CHICKEN
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Chicken,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (32.0, 34.0),
                path: "Chicken/Hit (32x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Chicken,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 13,
                tile_size: (32.0, 34.0),
                path: "Chicken/Idle (32x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Chicken,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 14,
                tile_size: (32.0, 34.0),
                path: "Chicken/Run (32x34).png",
            },
            // DUCK
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Duck,
                    anim_ty: EnemyAnimationStates::Fall,
                },
                frames: 1,
                tile_size: (36.0, 36.0),
                path: "Duck/Fall (36x36).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Duck,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (36.0, 36.0),
                path: "Duck/Hit (36x36).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Duck,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 10,
                tile_size: (36.0, 36.0),
                path: "Duck/Idle (36x36).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Duck,
                    anim_ty: EnemyAnimationStates::Jump,
                },
                frames: 1,
                tile_size: (36.0, 36.0),
                path: "Duck/Jump (36x36).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Duck,
                    anim_ty: EnemyAnimationStates::JumpAnticipation,
                },
                frames: 4,
                tile_size: (36.0, 36.0),
                path: "Duck/Jump Anticipation (36x36).png",
            },
            // FAT BIRD
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::FatBird,
                    anim_ty: EnemyAnimationStates::Fall,
                },
                frames: 4,
                tile_size: (40.0, 48.0),
                path: "FatBird/Fall (40x48).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::FatBird,
                    anim_ty: EnemyAnimationStates::Ground,
                },
                frames: 4,
                tile_size: (40.0, 48.0),
                path: "FatBird/Ground (40x48).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::FatBird,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (40.0, 48.0),
                path: "FatBird/Hit (40x48).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::FatBird,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 8,
                tile_size: (40.0, 48.0),
                path: "FatBird/Idle (40x48).png",
            },
            // GHOST
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Ghost,
                    anim_ty: EnemyAnimationStates::Appear,
                },
                frames: 4,
                tile_size: (44.0, 30.0),
                path: "Ghost/Appear (44x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Ghost,
                    anim_ty: EnemyAnimationStates::Disappear,
                },
                frames: 5,
                tile_size: (44.0, 30.0),
                path: "Ghost/Disappear (44x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Ghost,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (44.0, 30.0),
                path: "Ghost/Hit (44x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Ghost,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 10,
                tile_size: (44.0, 30.0),
                path: "Ghost/Idle (44x30).png",
            },
            // MUSHROOM
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Mushroom,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Mushroom/Hit.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Mushroom,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 14,
                tile_size: (32.0, 32.0),
                path: "Mushroom/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Mushroom,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 16,
                tile_size: (32.0, 32.0),
                path: "Mushroom/Run (32x32).png",
            },
            // PLANT
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Plant,
                    anim_ty: EnemyAnimationStates::Attack,
                },
                frames: 8,
                tile_size: (42.0, 42.0),
                path: "Plant/Attack (44x42).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Plant,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (44.0, 42.0),
                path: "Plant/Hit (44x42).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Plant,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 11,
                tile_size: (44.0, 42.0),
                path: "Plant/Idle (44x42).png",
            },
            // RADISH
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Radish,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (30.0, 38.0),
                path: "Radish/Hit (30x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Radish,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 6,
                tile_size: (30.0, 38.0),
                path: "Radish/Idle 1 (30x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Radish,
                    anim_ty: EnemyAnimationStates::Idle2,
                },
                frames: 9,
                tile_size: (30.0, 38.0),
                path: "Radish/Idle 2 (30x38).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Radish,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 12,
                tile_size: (30.0, 38.0),
                path: "Radish/Run (30x38).png",
            },
            // RHINO
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rhino,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (52.0, 34.0),
                path: "Rhino/Hit (52x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rhino,
                    anim_ty: EnemyAnimationStates::HitWall,
                },
                frames: 4,
                tile_size: (52.0, 34.0),
                path: "Rhino/Hit Wall (52x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rhino,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 11,
                tile_size: (52.0, 34.0),
                path: "Rhino/Idle (52x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rhino,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 6,
                tile_size: (52.0, 34.0),
                path: "Rhino/Run (52x34).png",
            },
            // ROCKS
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rocks1,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 1,
                tile_size: (38.0, 34.0),
                path: "Rocks/Rock1_Hit.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rocks1,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 14,
                tile_size: (38.0, 34.0),
                path: "Rocks/Rock1_Idle (38x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rocks1,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 14,
                tile_size: (38.0, 34.0),
                path: "Rocks/Rock1_Run (38x34).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rocks2,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 1,
                tile_size: (32.0, 28.0),
                path: "Rocks/Rock2_Hit (32x28).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rocks2,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 13,
                tile_size: (32.0, 28.0),
                path: "Rocks/Rock2_Idle (32x28).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rocks2,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 14,
                tile_size: (32.0, 28.0),
                path: "Rocks/Rock2_Run (32x28).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rocks3,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (22.0, 18.0),
                path: "Rocks/Rock3_Hit (22x18).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rocks3,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 11,
                tile_size: (22.0, 18.0),
                path: "Rocks/Rock3_Idle (22x18).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Rocks3,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 14,
                tile_size: (22.0, 18.0),
                path: "Rocks/Rock3_Run (22x18).png",
            },
            // SKULL
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Skull,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (52.0, 54.0),
                path: "Skull/Hit (52x54).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Skull,
                    anim_ty: EnemyAnimationStates::HitWall,
                },
                frames: 7,
                tile_size: (52.0, 54.0),
                path: "Skull/Hit Wall 1 (52x54).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Skull,
                    anim_ty: EnemyAnimationStates::HitWall2,
                },
                frames: 7,
                tile_size: (52.0, 54.0),
                path: "Skull/Hit Wall 2 (52x54).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Skull,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 8,
                tile_size: (52.0, 54.0),
                path: "Skull/Idle 1 (52x54).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Skull,
                    anim_ty: EnemyAnimationStates::Idle2,
                },
                frames: 8,
                tile_size: (52.0, 54.0),
                path: "Skull/Idle 2 (52x54).png",
            },
            // SLIME
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Slime,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (44.0, 30.0),
                path: "Slime/Hit (44x30).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Slime,
                    anim_ty: EnemyAnimationStates::IdleRun,
                },
                frames: 10,
                tile_size: (44.0, 30.0),
                path: "Slime/Idle-Run (44x30).png",
            },
            // SNAIL
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Snail,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (38.0, 24.0),
                path: "Snail/Hit (38x24).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Snail,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 15,
                tile_size: (38.0, 24.0),
                path: "Snail/Idle (38x24).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Snail,
                    anim_ty: EnemyAnimationStates::ShellIdle,
                },
                frames: 6,
                tile_size: (38.0, 24.0),
                path: "Snail/Shell Idle (38x24).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Snail,
                    anim_ty: EnemyAnimationStates::ShellTopHit,
                },
                frames: 5,
                tile_size: (38.0, 24.0),
                path: "Snail/Shell Top Hit (38x24).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Snail,
                    anim_ty: EnemyAnimationStates::ShellWallHit,
                },
                frames: 4,
                tile_size: (38.0, 24.0),
                path: "Snail/Shell Wall Hit (38x24).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Snail,
                    anim_ty: EnemyAnimationStates::SnailWithoutShell,
                },
                frames: 1,
                tile_size: (38.0, 24.0),
                path: "Snail/Snail without shell.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Snail,
                    anim_ty: EnemyAnimationStates::Walk,
                },
                frames: 10,
                tile_size: (38.0, 24.0),
                path: "Snail/Walk (38x24).png",
            },
            // TRUNK
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Trunk,
                    anim_ty: EnemyAnimationStates::Attack,
                },
                frames: 11,
                tile_size: (64.0, 32.0),
                path: "Trunk/Attack (64x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Trunk,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (64.0, 32.0),
                path: "Trunk/Hit (64x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Trunk,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 18,
                tile_size: (64.0, 32.0),
                path: "Trunk/Idle (64x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Trunk,
                    anim_ty: EnemyAnimationStates::Run,
                },
                frames: 14,
                tile_size: (64.0, 32.0),
                path: "Trunk/Run (64x32).png",
            },
            // TURTLE
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Turtle,
                    anim_ty: EnemyAnimationStates::Hit,
                },
                frames: 5,
                tile_size: (44.0, 26.0),
                path: "Turtle/Hit (44x26).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Turtle,
                    anim_ty: EnemyAnimationStates::Idle,
                },
                frames: 14,
                tile_size: (44.0, 26.0),
                path: "Turtle/Idle 1 (44x26).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Turtle,
                    anim_ty: EnemyAnimationStates::Idle2,
                },
                frames: 14,
                tile_size: (44.0, 26.0),
                path: "Turtle/Idle 2 (44x26).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Turtle,
                    anim_ty: EnemyAnimationStates::TurtleSpikesIn,
                },
                frames: 8,
                tile_size: (44.0, 26.0),
                path: "Turtle/Spikes in (44x26).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Turtle,
                    anim_ty: EnemyAnimationStates::TurtleSpikesOut,
                },
                frames: 8,
                tile_size: (44.0, 26.0),
                path: "Turtle/Spikes out (44x26).png",
            },
            // ~~ GENERIC ~~
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::Bullet,
                },
                frames: 1,
                tile_size: (16.0, 16.0),
                path: "Bee/Bullet.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::BulletPieces,
                },
                frames: 2,
                tile_size: (16.0, 16.0),
                path: "Bee/Bullet Pieces.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::BulletPieces,
                },
                frames: 2,
                tile_size: (16.0, 16.0),
                path: "Plant/Bullet Pieces.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::Bullet,
                },
                frames: 1,
                tile_size: (16.0, 16.0),
                path: "Plant/Bullet.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::GhostParticles,
                },
                frames: 1,
                tile_size: (48.0, 16.0),
                path: "Ghost/Ghost Particles (48x16).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::Leafs,
                },
                frames: 2,
                tile_size: (16.0, 16.0),
                path: "Radish/Leafs.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::RedParticle,
                },
                frames: 1,
                tile_size: (16.0, 16.0),
                path: "Skull/Red Particle.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::OrangeParticle,
                },
                frames: 1,
                tile_size: (16.0, 16.0),
                path: "Skull/Orange Particle.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::SlimeParticles,
                },
                frames: 1,
                tile_size: (62.0, 16.0),
                path: "Slime/Particles (62x16).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::Bullet,
                },
                frames: 1,
                tile_size: (16.0, 16.0),
                path: "Trunk/Bullet.png",
            },
            AnimationState {
                kv: EntTypeKey {
                    ty: EnemyTypeStates::Generic,
                    anim_ty: EnemyAnimationStates::BulletPieces,
                },
                frames: 2,
                tile_size: (16.0, 16.0),
                path: "Trunk/Bullet Pieces.png",
            },
        ],
        root_path: "pixels/Enemies/",
    }];
}
