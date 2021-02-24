pub enum Collider {
    // Player in front
    NoneBackground,

    // Player at back
    NoneForeground,

    Wall,
    Ceiling,
    Ground,
}

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