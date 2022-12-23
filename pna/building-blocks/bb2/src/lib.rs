use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Move {
    pub dir: Direction,
    pub steps: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

