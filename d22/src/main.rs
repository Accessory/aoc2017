use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;
use utils::hash_point_map::HashPointMap;
use utils::map_direction::MapDirection;
use utils::point::{MapPoint, MapWalker};

#[allow(dead_code)]
fn print_map(
    hash_point_map: &HashPointMap<u8>,
    start_x: i64,
    start_y: i64,
    height: i64,
    width: i64,
) {
    for y in (start_y..height).rev() {
        for x in start_x..width {
            match hash_point_map.get(&MapPoint::new(x, y)) {
                Some(point) => print!("{}", *point as char),
                None => print!("."),
            }
        }
        println!();
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut hash_point_map: HashPointMap<u8> = HashPointMap::default();
    let mut point_list = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut max_x = 0;
    let mut max_y = 0;

    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        max_x = max_x.max(line.len());
        max_y = max_y.max(row);
        for (column, ch) in line.chars().enumerate() {
            if ch == '#' {
                point_list.push(MapPoint::new(column as i64, row as i64));
            }
        }
    }

    // Prepare
    for point in point_list {
        hash_point_map.push(
            MapPoint::new(point.x as i64, max_y as i64 - point.y as i64),
            b'#',
        );
    }

    let mid_x = (max_x / 2) as i64;
    let mid_y = (max_y / 2) as i64;

    let mut walker: MapWalker = MapWalker {
        direction: MapDirection::Up,
        position: MapPoint { x: mid_x, y: mid_y },
    };

    // Solve
    let mut result: usize = 0;
    for _ in 0..10000 {
        match hash_point_map.get(&walker.position) {
            Some(_) => {
                hash_point_map.remove(&walker.position);
                walker.turn_right();
            }
            None => {
                result += 1;
                hash_point_map.push(walker.position, b'#');
                walker.turn_left();
            }
        }
        // dbg!(&walker);

        walker.r#move()
    }

    // Result
    // print_map(&hash_point_map, -4, -4, 9, 9);

    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut hash_point_map: HashPointMap<u8> = HashPointMap::default();
    let mut point_list = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut max_x = 0;
    let mut max_y = 0;

    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        max_x = max_x.max(line.len());
        max_y = max_y.max(row);
        for (column, ch) in line.chars().enumerate() {
            if ch == '#' {
                point_list.push(MapPoint::new(column as i64, row as i64));
            }
        }
    }

    // Prepare
    for point in point_list {
        hash_point_map.push(
            MapPoint::new(point.x as i64, max_y as i64 - point.y as i64),
            b'I',
        );
    }

    let mid_x = (max_x / 2) as i64;
    let mid_y = (max_y / 2) as i64;

    let mut walker: MapWalker = MapWalker {
        direction: MapDirection::Up,
        position: MapPoint { x: mid_x, y: mid_y },
    };

    // Solve
    let mut result: usize = 0;
    for _ in 0..10000000 {
        match hash_point_map.get(&walker.position) {
            Some(data) => match data {
                b'W' => {
                    hash_point_map.push(walker.position, b'I');
                    result += 1;
                }
                b'I' => {
                    hash_point_map.push(walker.position, b'F');
                    walker.turn_right();
                }
                b'F' => {
                    hash_point_map.remove(&walker.position);
                    walker.turn_around();
                }
                _ => panic!("Should not happen"),
            },
            None => {
                hash_point_map.push(walker.position, b'W');
                walker.turn_left();
            }
        }
        // dbg!(&walker);

        walker.r#move()
    }

    // Result
    // print_map(&hash_point_map, -4, -4, 9, 9);

    println!("Result is {}", result);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
    run2(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
