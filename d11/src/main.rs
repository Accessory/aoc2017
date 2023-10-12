use std::fs;

use utils::get_input_path;

enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

fn cube_distance(q: i64, r: i64, s: i64) -> i64 {
    let p1 = q.abs();
    let p2 = s.abs();
    let p3 = r.abs();

    (p1 + p2 + p3) / 2
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "n" => Direction::North,
            "ne" => Direction::NorthEast,
            "se" => Direction::SouthEast,
            "s" => Direction::South,
            "sw" => Direction::SouthWest,
            "nw" => Direction::NorthWest,
            _ => panic!("Should not be here: {value}"),
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut q: i64 = 0;
    let mut r: i64 = 0;
    let mut s: i64 = 0;
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    let split = line.split(",");

    for dir in split {
        let de: Direction = dir.into();
        match de {
            Direction::North => {
                r -= 1;
                s += 1;
            }
            Direction::NorthEast => {
                q += 1;
                r -= 1;
            }
            Direction::SouthEast => {
                q += 1;
                s -= 1;
            }
            Direction::South => {
                r += 1;
                s -= 1;
            }
            Direction::SouthWest => {
                q -= 1;
                r += 1;
            }
            Direction::NorthWest => {
                q -= 1;
                s += 1;
            }
        }
    }

    // Result
    let distance: i64 = cube_distance(q, r, s);
    println!(
        "Result is of part 1 is -> q: {q} r: {r} s: {s} distance: {} ",
        distance
    );
}

fn run2(input_file: &str) {
    // Preamble
    let mut q: i64 = 0;
    let mut r: i64 = 0;
    let mut s: i64 = 0;
    let mut max_distance: i64 = 0;
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    let split = line.split(",");

    for dir in split {
        let de: Direction = dir.into();
        match de {
            Direction::North => {
                r -= 1;
                s += 1;
            }
            Direction::NorthEast => {
                q += 1;
                r -= 1;
            }
            Direction::SouthEast => {
                q += 1;
                s -= 1;
            }
            Direction::South => {
                r += 1;
                s -= 1;
            }
            Direction::SouthWest => {
                q -= 1;
                r += 1;
            }
            Direction::NorthWest => {
                q -= 1;
                s += 1;
            }
        }

        max_distance = max_distance.max(cube_distance(q, r, s));
    }

    // Result
    println!(
        "Result is of part 2 is -> q: {q} r: {r} s: {s} distance: {} ",
        max_distance
    );
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
