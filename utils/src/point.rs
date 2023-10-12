use crate::{map_direction::MapDirection, utils::manhatten_distance};

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct MapPoint {
    pub x: i64,
    pub y: i64,
}

impl MapPoint {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapWalker {
    pub direction: MapDirection,
    pub position: MapPoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapInfoWalker<T> {
    pub map_walker: MapWalker,
    pub info: T,
}

impl<T> MapInfoWalker<T> {
    pub fn r#move(&mut self) {
        self.map_walker.r#move();
    }
}

impl MapWalker {
    pub fn r#move(&mut self) {
        match self.direction {
            MapDirection::Up => self.position.move_up(),
            MapDirection::Right => self.position.move_right(),
            MapDirection::Down => self.position.move_down(),
            MapDirection::Left => self.position.move_left(),
        };
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            MapDirection::Up => MapDirection::Left,
            MapDirection::Right => MapDirection::Up,
            MapDirection::Down => MapDirection::Right,
            MapDirection::Left => MapDirection::Down,
        }
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            MapDirection::Up => MapDirection::Right,
            MapDirection::Right => MapDirection::Down,
            MapDirection::Down => MapDirection::Left,
            MapDirection::Left => MapDirection::Up,
        }
    }

    pub fn turn_around(&mut self) {
        self.direction = match self.direction {
            MapDirection::Up => MapDirection::Down,
            MapDirection::Right => MapDirection::Left,
            MapDirection::Down => MapDirection::Up,
            MapDirection::Left => MapDirection::Right,
        }
    }
}

impl MapPoint {
    pub fn generate_non_diagonal_neigbors(&self) -> Vec<MapPoint> {
        vec![
            MapPoint {
                x: self.x - 1,
                y: self.y,
            },
            MapPoint {
                x: self.x,
                y: self.y - 1,
            },
            MapPoint {
                x: self.x,
                y: self.y + 1,
            },
            MapPoint {
                x: self.x + 1,
                y: self.y,
            },
        ]
    }
    pub fn generate_neigbors(&self) -> Vec<MapPoint> {
        vec![
            MapPoint {
                x: self.x - 1,
                y: self.y - 1,
            },
            MapPoint {
                x: self.x - 1,
                y: self.y,
            },
            MapPoint {
                x: self.x - 1,
                y: self.y + 1,
            },
            MapPoint {
                x: self.x,
                y: self.y - 1,
            },
            MapPoint {
                x: self.x,
                y: self.y + 1,
            },
            MapPoint {
                x: self.x + 1,
                y: self.y - 1,
            },
            MapPoint {
                x: self.x + 1,
                y: self.y,
            },
            MapPoint {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn next_from_direction(&self, direction: &MapDirection) -> Self {
        match direction {
            MapDirection::Up => self.next_up(),
            MapDirection::Right => self.next_right(),
            MapDirection::Down => self.next_down(),
            MapDirection::Left => self.next_left(),
        }
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_up(&mut self) {
        self.y += 1;
    }

    pub fn move_down(&mut self) {
        self.y -= 1;
    }

    pub fn move_right_down(&mut self) {
        self.x += 1;
        self.y -= 1;
    }

    pub fn move_down_right(&mut self) {
        self.x += 1;
        self.y -= 1;
    }

    pub fn manhatten_distance(&self, point: Self) -> i64 {
        manhatten_distance(self.x, self.y, point.x, point.y)
    }

    fn next_up(&self) -> MapPoint {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn next_left(&self) -> MapPoint {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn next_down(&self) -> MapPoint {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn next_right(&self) -> MapPoint {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}
