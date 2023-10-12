use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

pub mod utils;
pub mod map_direction;
pub mod hash_point_map;
pub mod point;
pub mod grid_direction;
pub mod grid_point;
pub mod grid;
pub mod map;

pub fn get_input_path(src_path: &str) -> PathBuf {
    let file_path = Path::new(src_path);
    if Path::exists(file_path) {
        file_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join("input.txt")
    } else {
        current_dir().unwrap().join("input").join("input.txt")
    }
}

pub fn get_test_input_path(src_path: &str) -> PathBuf {
    let file_path = Path::new(src_path);
    if Path::exists(file_path) {
        file_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join("input_test.txt")
    } else {
        current_dir().unwrap().join("input").join("input_test.txt")
    }
}

pub fn get_test_input_2_path(src_path: &str) -> PathBuf {
    let file_path = Path::new(src_path);
    if Path::exists(file_path) {
        file_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join("input_test_2.txt")
    } else {
        current_dir().unwrap().join("input").join("input_test.txt")
    }
}

#[cfg(test)]
mod tests {
    use crate::get_input_path;
    use crate::get_test_input_path;
    use crate::map::Map;
    use crate::map::Point;

    #[test]
    fn test_get_test_input_path() {
        println!("{}", get_test_input_path(file!()).to_string_lossy());
    }

    #[test]
    fn test_get_input_path() {
        println!("{}", get_input_path(file!()).to_string_lossy());
    }

    #[test]
    fn test_map() {
        let mut map = Map::new(10, 10);
        map.set(5,5, 10);
        let rtn = map.get(5, 5);
        let rtn2 = map.get_from_point(Point{ x: 5, y: 5 });
        assert!(rtn == 10);
        assert!(rtn2 == 10);
    }

    #[test]
    fn test_point() {
        let p1 = Point{ x: 0, y: 0 };
        let p2 = Point{ x: 5, y: 5 };
        let distance = p1.manhatten_distance(p2);
        assert!(distance == 10);
    }



}
