#![feature(linked_list_cursors)]
use std::fs;

use utils::get_input_path;

fn run(input_file: &str) {
    // Parse
    let steps: usize = fs::read_to_string(input_file)
        .unwrap()
        .trim()
        .to_string()
        .parse()
        .unwrap();

    // Prepare
    let mut ring: Vec<usize> = Vec::with_capacity(2018);
    ring.push(0);

    let mut current_positon = 0;

    // Solve
    for i in 01..2018 {
        current_positon = ((current_positon + steps) % ring.len()) + 1;
        ring.insert(current_positon, i);
    }

    // Result
    println!("Result is {}", ring.get(current_positon + 1).unwrap());
}

fn run2(input_file: &str) {
    //     // Parse
    let steps: usize = fs::read_to_string(input_file)
        .unwrap()
        .trim()
        .to_string()
        .parse()
        .unwrap();

    // Prepare
    let mut current_positon = 0;
    let mut result = 60000000;

    // Solve
    for i in 1..50000000 {
        current_positon = ((current_positon + steps) % i) + 1;
        if current_positon == 1 {
            result = i;
        }
    }
    println!("Result of part 2 is: {result}");
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
