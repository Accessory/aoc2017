use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    const GEN_A_FACTOR: u64 = 16807;
    const GEN_B_FACTOR: u64 = 48271;

    const GEN_DIVIDER: u64 = 2147483647;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut gen_a: u64 = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .skip(4)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut gen_b: u64 = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .skip(4)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    // Solve
    let mut pairs: usize = 0;

    // println!();
    // for _ in 0..5 {
    for _ in 0..40_000_000 {
        gen_a = (GEN_A_FACTOR * gen_a) % GEN_DIVIDER;
        gen_b = (GEN_B_FACTOR * gen_b) % GEN_DIVIDER;

        if (gen_a & 0xFFFF) == (gen_b & 0xFFFF) {
            pairs += 1;
            // println!("{gen_a} - {gen_b}");
            // println!("{:016b} - {:016b}", (gen_a & 0xFFFF), (gen_b & 0xFFFF));
        }
    }

    // Result

    println!("Result is {}", pairs);
}

fn run2(input_file: &str) {
    // Preamble
    const GEN_A_FACTOR: u64 = 16807;
    const GEN_B_FACTOR: u64 = 48271;

    const GEN_DIVIDER: u64 = 2147483647;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut gen_a: u64 = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .skip(4)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut gen_b: u64 = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .skip(4)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    // Solve
    let mut pairs: usize = 0;

    // for _ in 0..5 {
    for _ in 0..5_000_000 {
        gen_a = (GEN_A_FACTOR * gen_a) % GEN_DIVIDER;
        while gen_a % 4 != 0 {
            gen_a = (GEN_A_FACTOR * gen_a) % GEN_DIVIDER;
        }

        gen_b = (GEN_B_FACTOR * gen_b) % GEN_DIVIDER;
        while gen_b % 8 != 0 {
            gen_b = (GEN_B_FACTOR * gen_b) % GEN_DIVIDER;
        }

        if (gen_a & 0xFFFF) == (gen_b & 0xFFFF) {
            pairs += 1;
            // println!("{gen_a} - {gen_b}");
            // println!("{:016b} - {:016b}", (gen_a & 0xFFFF), (gen_b & 0xFFFF));
        }
    }

    // Result
    println!("Result is {}", pairs);
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
