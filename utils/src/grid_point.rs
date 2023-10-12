use crate::grid_direction::GridDirection;

#[derive(Debug, Clone, Copy, Default)]
pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

impl GridPoint {
    pub fn generate_non_diagonal_neigbors(&self) -> Vec<GridPoint> {
        let mut rtn = Vec::with_capacity(4);
        if self.x > 0 {
            rtn.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }
        rtn.push(Self {
            x: self.x + 1,
            y: self.y,
        });

        if self.y > 0 {
            rtn.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }

        rtn.push(Self {
            x: self.x,
            y: self.y + 1,
        });

        rtn
    }
    pub fn generate_non_diagonal_neigbors_with_check(
        &self,
        max_x: usize,
        max_y: usize,
    ) -> Vec<GridPoint> {
        let mut rtn = Vec::with_capacity(4);
        if self.x > 0 {
            rtn.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.x < max_x - 1 {
            rtn.push(Self {
                x: self.x + 1,
                y: self.y,
            });
        }

        if self.y > 0 {
            rtn.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.y < max_y - 1 {
            rtn.push(Self {
                x: self.x,
                y: self.y + 1,
            });
        }
        rtn
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn next_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn next_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn next_up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn next_down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn move_direction(&mut self, direction: &GridDirection) {
        match direction {
            GridDirection::Up => self.move_up(),
            GridDirection::Right => self.move_right(),
            GridDirection::Down => self.move_down(),
            GridDirection::Left => self.move_left(),
        }
    }

    pub fn next_by_direction_with_check(
        &self,
        direction: &GridDirection,
        max_x: usize,
        max_y: usize,
    ) -> Option<Self> {
        match direction {
            GridDirection::Up => self.next_up_checked(),
            GridDirection::Right => self.next_right_checked(max_x),
            GridDirection::Down => self.next_down_checked(max_y),
            GridDirection::Left => self.next_left_checked(),
        }
    }

    pub fn next_by_direction(&self, direction: &GridDirection) -> Self {
        match direction {
            GridDirection::Up => self.next_up(),
            GridDirection::Right => self.next_right(),
            GridDirection::Down => self.next_down(),
            GridDirection::Left => self.next_left(),
        }
    }

    fn next_up_checked(&self) -> Option<GridPoint> {
        if self.y != 0 {
            return Some(self.next_up());
        }
        None
    }
    fn next_right_checked(&self, max_x: usize) -> Option<GridPoint> {
        if self.x != max_x - 1 {
            return Some(self.next_right());
        }
        None
    }
    fn next_down_checked(&self, max_y: usize) -> Option<GridPoint> {
        if self.y != max_y - 1 {
            return Some(self.next_down());
        }
        None
    }

    fn next_left_checked(&self) -> Option<GridPoint> {
        if self.x != 0 {
            return Some(self.next_left());
        }
        None
    }
}
