use core::panic;
use std::{
    fs,
    ops::{Add, Div, Sub},
};

use utils::{get_input_path, hash_point_map::HashPointMap, point::MapPoint};

fn run(input_file: &str) {
    // Preamble
    let result;
    let mut dimension: i64 = 1;
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let number: i64 = line.parse().unwrap();
    // Prepare
    while dimension.pow(2) < number {
        dimension += 2;
    }

    // Solve
    let start: i64 = dimension.sub(2).pow(2).add(1);
    let div_to_start: i64 = number - start;

    if div_to_start < (dimension - 1) {
        let position = div_to_start.sub(dimension.div(2)).add(1);
        result = utils::utils::manhatten_distance(0, 0, position, dimension.div(2))
    } else if div_to_start < ((dimension - 1) * 2) {
        let position = div_to_start.sub(dimension - 1).sub(dimension.div(2)).add(1);
        result = utils::utils::manhatten_distance(0, 0, position, dimension.div(2))
    } else if div_to_start < ((dimension - 1) * 3) {
        let position = div_to_start
            .sub((dimension - 1) * 2)
            .sub(dimension.div(2))
            .add(1);
        result = utils::utils::manhatten_distance(0, 0, position, dimension.div(2))
    } else if div_to_start < ((dimension - 1) * 4) {
        let position = div_to_start
            .sub((dimension - 1) * 3)
            .sub(dimension.div(2))
            .add(1);
        result = utils::utils::manhatten_distance(0, 0, position, dimension.div(2))
    } else {
        panic!("Something is wrong number: {number} - div_to_start: {div_to_start} - dimension: {dimension}");
    }

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut point_map: HashPointMap<i64> = HashPointMap::default();
    let mut dimension: i64 = 1;
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let number: i64 = line.parse().unwrap();
    // Prepare
    // while dimension.pow(2) < number {
    // dimension += 2;
    // }

    let mut current_point = MapPoint { x: 0, y: 0 };

    point_map.push(current_point, 1);



    // Solve
    'outer: loop {
        current_point.move_right_down();
        dimension += 2;

        let to_walk = dimension - 1;

        for _ in 0..to_walk {
            current_point.move_up();

            let neigbors = current_point.generate_neigbors();

            let value: i64 = neigbors
                .iter()
                .map(|n| point_map.get(n).unwrap_or(&0))
                .sum();

            point_map.push(current_point, value);
            if value > number {
                break 'outer;
            }
        }

        for _ in 0..to_walk {
            current_point.move_left();

            let neigbors = current_point.generate_neigbors();

            let value: i64 = neigbors
                .iter()
                .map(|n| point_map.get(n).unwrap_or(&0))
                .sum();

            point_map.push(current_point, value);
            if value > number {
                break 'outer;
            }
        }

        for _ in 0..to_walk {
            current_point.move_down();

            let neigbors = current_point.generate_neigbors();

            let value: i64 = neigbors
                .iter()
                .map(|n| point_map.get(n).unwrap_or(&0))
                .sum();

            point_map.push(current_point, value);
            if value > number {
                break 'outer;
            }
        }

        for _ in 0..to_walk {
            current_point.move_right();

            let neigbors = current_point.generate_neigbors();

            let value: i64 = neigbors
                .iter()
                .map(|n| point_map.get(n).unwrap_or(&0))
                .sum();

            point_map.push(current_point, value);
            if value > number {
                break 'outer;
            }
        }
    }

    // Result
    println!("Result is {}", point_map.get(&current_point).unwrap());
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
