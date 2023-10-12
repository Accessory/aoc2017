use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug)]
#[allow(dead_code)]
struct Disc {
    name: String,
    number: usize,
    connections: Vec<String>,
}

fn calculate_tower(connection: &str, disc_map: &HashMap<String, Disc>) -> usize {
    let disc = disc_map.get(connection).unwrap();
    let mut result = disc.number;
    let mut results = Vec::new();

    if disc.connections.len() == 0 {
        return disc.number;
    }

    for connection in disc.connections.iter() {
        let part_result = calculate_tower(connection, disc_map);
        // println!("Part Result: Name {connection} Number: {part_result} Deep: {deep}");
        result += part_result;
        results.push(part_result);
    }

    let min = results.iter().min().unwrap();
    let max = results.iter().max().unwrap();

    if min == max {
        return result
    }

    let diff = max - min;

    let idx = results.iter().enumerate().find(|i: &(usize, &usize)| i.1 == max).unwrap().0;
    let unbalanced_stack = disc.connections.get(idx).unwrap();
    let disc = disc_map.get(unbalanced_stack).unwrap();

    println!("Result part 2 is {}", disc.number - diff);
    return result - diff
}

fn run(input_file: &str) {
    // Preamble
    let mut names: HashSet<String> = HashSet::new();
    let mut disc_map: HashMap<String, Disc> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let split: Vec<String> = line
            .split([' ', ',', '(', ')'])
            .filter(|f| !f.is_empty())
            .map(|i| i.to_string())
            .collect();
        let name = split[0].clone();
        let number = split[1].parse().unwrap();
        let connections: Vec<String> = if split.len() > 3 {
            split[3..].iter().map(|i| i.clone()).collect()
        } else {
            Vec::new()
        };

        names.insert(name.clone());

        let disc = Disc {
            name: name.clone(),
            number,
            connections,
        };

        disc_map.insert(name, disc);
    }

    // Solve
    for (_, disc) in disc_map.iter() {
        for connection in disc.connections.iter() {
            names.remove(connection);
        }
    }

    // Result
    let result = names.iter().next().unwrap();
    println!("Result of part 1 is: {result}");
}

fn run2(input_file: &str) {
    // Preamble
    let mut names: HashSet<String> = HashSet::new();
    let mut disc_map: HashMap<String, Disc> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let split: Vec<String> = line
            .split([' ', ',', '(', ')'])
            .filter(|f| !f.is_empty())
            .map(|i| i.to_string())
            .collect();
        let name = split[0].clone();
        let number = split[1].parse().unwrap();
        let connections: Vec<String> = if split.len() > 3 {
            split[3..].iter().map(|i| i.clone()).collect()
        } else {
            Vec::new()
        };

        names.insert(name.clone());

        let disc = Disc {
            name: name.clone(),
            number,
            connections,
        };

        disc_map.insert(name, disc);
    }

    // Solve
    for (_, disc) in disc_map.iter() {
        for connection in disc.connections.iter() {
            names.remove(connection);
        }
    }

    // Result 1
    let result_part1 = names.iter().next().unwrap();

    // Solve 2
    // let result =
     calculate_tower(result_part1, &disc_map);

    // println!("The Result of part 2 is: {}", result);
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
