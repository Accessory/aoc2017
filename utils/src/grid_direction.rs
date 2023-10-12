#[derive(Debug, Clone, Copy)]
pub enum GridDirection {
    Up,
    Right,
    Down,
    Left,
}

impl GridDirection {
    pub fn get_directions() -> [GridDirection; 4] {
        [
            GridDirection::Up,
            GridDirection::Right,
            GridDirection::Down,
            GridDirection::Left,
        ]
    }

    pub fn get_int_char(&self) -> char {
        match self {
            GridDirection::Up => '0',
            GridDirection::Right => '1',
            GridDirection::Down => '2',
            GridDirection::Left => '3',
        }
    }

    pub fn from_int_char(c:char) -> Self {
        match c {
            '0' => GridDirection::Up,
            '1' => GridDirection::Right,
            '2' => GridDirection::Down,
            '3' => GridDirection::Left,
            _ => panic!("Not a valid direction {c}"),
        }
    }
}