use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut result: i64 = 0;
    let mut current_position: i64 = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut jumps: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Solve
    loop {
        if current_position < 0 || current_position as usize >= jumps.len() {
            break;
        }

        result += 1;
        let old = jumps[current_position as usize];
        jumps[current_position as usize] = old + 1;
        current_position += old;
    }

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result: i64 = 0;
    let mut current_position: i64 = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut jumps: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Solve
    loop {
        if current_position < 0 || current_position as usize >= jumps.len() {
            break;
        }

        result += 1;
        let old = jumps[current_position as usize];
        if old < 3 {
            jumps[current_position as usize] = old + 1;
        } else {
            jumps[current_position as usize] = old - 1;
        }
        current_position += old;
    }

    // Result
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
