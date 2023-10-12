use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    fs,
    hash::{Hasher, Hash},
    usize,
};

use utils::get_input_path;

fn reallocate(banks: &mut Vec<usize>) {
    let mut max = *banks.iter().max().unwrap();
    let mut reallocate = false;
    let mut current_position = 0;
    let go_back_value = banks.len() - 1;

    while max != 0 {
        if reallocate {
            banks[current_position] = banks[current_position] + 1;
            max -= 1;
        } else {
            if banks[current_position] == max {
                reallocate = true;
                banks[current_position] = 0;
            }
        }
        current_position = if current_position == go_back_value {
            0
        } else {
            current_position + 1
        };
    }
}

fn create_hash(banks: &Vec<usize>) -> u64 {
    let mut hasher = DefaultHasher::new();
    banks.hash(&mut hasher);
    hasher.finish()
}

fn run(input_file: &str) {
    // Preamble
    let mut cycle_count = 0;
    let mut cache = HashSet::new();

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let mut banks: Vec<usize> = line
        .split(['\t', ' '])
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    // Solve

    loop {
        let hash_value = create_hash(&banks);
        if !cache.insert(hash_value) {
            break;
        }

        reallocate(&mut banks);

        cycle_count += 1;
    }

    // Result
    println!("Result of part 1 is {}", cycle_count);
}

fn run2(input_file: &str) {
        // Preamble
        let mut cycle_count = 0;
        let mut cache = HashSet::new();
        let mut hash_list = Vec::new();
    
        // Parse
        let line = fs::read_to_string(input_file).unwrap().trim().to_string();
        let mut banks: Vec<usize> = line
            .split(['\t', ' '])
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
        // Solve
    
        loop {
            let hash_value = create_hash(&banks);
            hash_list.push(hash_value);
            if !cache.insert(hash_value) {
                break;
            }
    
            reallocate(&mut banks);
    
            cycle_count += 1;
        }

        let last_hash = *hash_list.last().unwrap();
        let mut distance = 0;

        for i in (0..hash_list.len()-1).rev() {
            if last_hash == hash_list[i]{
                distance = cycle_count - i;
                break;
            }
        }
    
        // Result
        println!("Result of part 2 is {}", distance);
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
