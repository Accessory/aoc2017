use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn map_lines(item: Result<String, std::io::Error>) -> Vec<usize> {
    item.unwrap()
        .split([' ', ','])
        .filter(|s| !s.is_empty())
        .skip(2)
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

fn run(input_file: &str) {
    // Preamble
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let programm_list: Vec<Vec<usize>> = reader.lines().map(map_lines).collect();

    // Prepare
    let mut queue: Vec<usize> = vec![0];
    let mut seen: HashSet<usize> = HashSet::new();

    while let Some(current_id) = queue.pop() {
        if !seen.insert(current_id) {
            continue;
        }

        queue.extend(programm_list.get(current_id).unwrap());
    }

    // Solve
    // Result
    println!("Result of part 1 is {}", seen.len());
}

fn run2(input_file: &str) {
    // Preamble
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let programm_list: Vec<Vec<usize>> = reader.lines().map(map_lines).collect();

    // Prepare
    
    let mut seen: HashSet<usize> = HashSet::new();
    let mut group_count:usize = 0;

    for i in 0..programm_list.len(){
        if seen.contains(&i) {
            continue;
        }

        let mut queue: Vec<usize> = vec![i];
        group_count += 1;

        while let Some(current_id) = queue.pop() {
            if !seen.insert(current_id) {
                continue;
            }
    
            queue.extend(programm_list.get(current_id).unwrap());
        }
    }
    // Solve
    // Result
    println!("Result of part 2 is {}", group_count);
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
