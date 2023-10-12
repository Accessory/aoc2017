use std::fs;

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut result: u32 = 0;
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    for i in 0..line.len() {
        let mut p2 = i + 1;
        if p2 == line.len() {
            p2 = 0;
        }

        if line.chars().nth(i).unwrap() == line.chars().nth(p2).unwrap() {
            let to_add = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            result += to_add;
        }
    }

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result: u32 = 0;
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let helf_len = line.len() / 2;

    // Solve
    for i in 0..line.len() {
        let p2 = (i + helf_len) % line.len();

        if line.chars().nth(i).unwrap() == line.chars().nth(p2).unwrap() {
            let to_add = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            result += to_add;
        }
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

    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
