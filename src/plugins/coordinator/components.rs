#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum BackgroundType {
    Blue,
    Brown,
    Gray,
    Green,
    Pink,
    Purple,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum EnemyType {
    AngryPig,
    Bat,
    Bee,
    BlueBird,
    Bunny,
    Chameleon,
    Chicken,
    Duck,
    FatBird,
    Ghost,
    Mushroom,
    Plant,
    Radish,
    Rino,
    Rocks,
    Skull,
    Slime,
    Snail,
    Trunk,
    Turtle,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ItemsType {
    Boxes,
    Checkpoints,
    Fruits,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum BoxesType {
    Box1,
    Box2,
    Box3,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum CheckpointsType {
    Checkpoint,
    End,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum FruitsType {
    Apple,
    Bananas,
    Cherries,
    Collected,
    Kiwi,
    Melon,
    Orange,
    Pineapple,
    Strawberry,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TrapsType {
    Arrow,
    Blocks,
    FallingPlatforms,
    Fan,
    Fire,
    Platforms,
    RockHead,
    SandMudIce,
    Saw,
    SpikedBall,
    SpikeHead,
    Spikes,
    Trampoline,
}
