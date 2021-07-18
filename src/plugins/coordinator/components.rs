use super::EnemyTypeStates;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CoordinatorCommands {
    EnemyTransform(EnemyTypeStates),
}

// TODO: Move this into respective dirss
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
