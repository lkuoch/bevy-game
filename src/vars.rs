pub mod players {
    pub mod mask_dude_anim {
        const ROOT: &str = "pixels/Players/Mask Dude/";

        pub const IDLE_PATH: &str = const_format::concatcp!(ROOT, "/Idle (32x32).png");
        pub const IDLE_FRAMES: usize = 11;

        pub const DOUBLE_JUMP_PATH: &str =
            const_format::concatcp!(ROOT, "/Double Jump (32x32).png");
        pub const DOUBLE_JUMP_FRAMES: usize = 6;

        pub const FALL_PATH: &str = const_format::concatcp!(ROOT, "/Fall (32x32).png");
        pub const FALL_FRAMES: usize = 1;

        pub const HIT_PATH: &str = const_format::concatcp!(ROOT, "/Hit (32x32).png");
        pub const HIT_FRAMES: usize = 7;

        pub const JUMP_PATH: &str = const_format::concatcp!(ROOT, "/Jump (32x32).png");
        pub const JUMP_FRAMES: usize = 1;

        pub const RUN_PATH: &str = const_format::concatcp!(ROOT, "/Run (32x32).png");
        pub const RUN_FRAMES: usize = 12;

        pub const WALL_JUMP_PATH: &str = const_format::concatcp!(ROOT, "/Wall Jump (32x32).png");
        pub const WALL_JUMP_FRAMES: usize = 12;
    }
}
