use std::collections::HashMap;

use crate::point::MapPoint;

#[derive(Debug, Default)]
pub struct HashPointMap<T> {
    pub data: HashMap<MapPoint, T>,
}

impl<T> HashPointMap<T> {
    pub fn get(&self, point: &MapPoint) -> Option<&T> {
        self.data.get(point)
    }

    pub fn push(&mut self, key: MapPoint, value: T) {
        self.data.insert(key, value);
    }

    pub fn remove(&mut self, point: &MapPoint) -> Option<T> {
        self.data.remove(point)
    }
}
