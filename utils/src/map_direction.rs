#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapDirection {
    Up,
    Right,
    Down,
    Left,
}

impl MapDirection {
    pub fn get_directions() -> [MapDirection; 4] {
        [
            MapDirection::Up,
            MapDirection::Right,
            MapDirection::Down,
            MapDirection::Left,
        ]
    }
}
