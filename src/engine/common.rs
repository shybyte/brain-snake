pub type GridCoordinate = i16;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GridPosition {
    pub x: GridCoordinate,
    pub y: GridCoordinate,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
