use crate::grid_point::GridPoint;

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn get_max_x(&self) -> usize {
        self.data.iter().map(|i| i.len()).max().unwrap_or(0)
    }

    pub fn get_max_y(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|i| i.get(x))
    }

    pub fn count_for(&self, value: &T) -> usize
    where
        T: PartialEq,
    {
        self.data.iter().flatten().filter(|i| *i == value).count()
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y][x] = value;
    }

    pub fn get_from_point(&self, point: &GridPoint) -> Option<&T> {
        self.get(point.x, point.y)
    }

    pub fn set_from_point(&mut self, point: &GridPoint, value: T) {
        self.set(point.x, point.y, value)
    }

    pub fn create_sub_grid(&self, x: usize, y: usize, width: usize, height: usize) -> Self
    where
        T: Copy,
    {
        let mut new_data: Vec<Vec<T>> = Vec::with_capacity(height);
        for y in y..y + width {
            let mut row = Vec::with_capacity(width);
            for x in x..x + width {
                let point = self.get(x, y).unwrap();
                row.push(*point);
            }
            new_data.push(row);
        }
        Grid { data: new_data }
    }

    pub fn get_sub_grid(&self, x: usize, y: usize, width: usize, height: usize) -> Grid<&T> {
        let mut new_data: Vec<Vec<&T>> = Vec::with_capacity(height);
        for y in y..y + width {
            let mut row = Vec::with_capacity(width);
            for x in x..x + width {
                let point = self.get(x, y).unwrap();
                row.push(point);
            }
            new_data.push(row);
        }
        Grid { data: new_data }
    }
}
