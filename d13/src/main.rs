use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, Clone, Copy)]
struct Scanner {
    pub current_postion: usize,
    dir_change: usize,
    pub depth: usize,
    goes_down: bool,
}

fn walk_all(scanner_map: &mut HashMap<usize, Scanner>) {
    scanner_map.iter_mut().for_each(|s| s.1.walk())
}

// fn reset_all(scanner_map: &mut HashMap<usize, Scanner>) {
//     scanner_map.iter_mut().for_each(|s| s.1.reset())
// }

fn calculate_hit_state(scanner_map: &HashMap<usize, Scanner>) -> HashSet<usize> {
    scanner_map
        .iter()
        .filter(|s| s.1.current_postion == 0)
        .map(|s| *s.0)
        .collect()
}

fn get_current_hit_state<'a>(
    delay: usize,
    cache: &'a mut Vec<HashSet<usize>>,
    scanner_map: &'a mut HashMap<usize, Scanner>,
) -> &'a HashSet<usize> {
    while cache.len() <= delay {
        let hit_state = calculate_hit_state(scanner_map);
        cache.push(hit_state);
        walk_all(scanner_map);
    }
    cache.get(delay).unwrap()
}

impl Scanner {
    fn new(depth: usize) -> Self {
        Self {
            current_postion: 0,
            dir_change: depth - 1,
            depth,
            goes_down: true,
        }
    }

    // fn reset(&mut self) {
    //     self.current_postion = 0;
    //     self.goes_down = true;
    // }

    fn walk(&mut self) {
        if self.goes_down {
            self.current_postion += 1;
            if self.dir_change == self.current_postion {
                self.goes_down = false;
            }
        } else {
            self.current_postion -= 1;
            if self.current_postion == 0 {
                self.goes_down = true;
            }
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut scanner_map = HashMap::new();
    let mut num_layer = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let mut split = line.split([':', ' ']);
        let layer: usize = split.next().unwrap().parse().unwrap();
        let depth: usize = split.skip(1).next().unwrap().parse().unwrap();
        num_layer = num_layer.max(layer);
        let scanner = Scanner::new(depth);
        scanner_map.insert(layer, scanner);
    }

    // Prepare
    let mut hits: Vec<usize> = Vec::new();

    // Solve
    for current_position in 0..=num_layer {
        if let Some(scanner) = scanner_map.get(&current_position) {
            if scanner.current_postion == 0 {
                hits.push(current_position);
            }
        }
        walk_all(&mut scanner_map);
    }
    // Result
    let mut result = 0;

    for hit in hits {
        let scanner = scanner_map.get(&hit).unwrap();
        result += scanner.depth * hit;
    }

    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut scanner_map = HashMap::new();
    let mut num_layer = 0;

    let mut delay: usize = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let mut split = line.split([':', ' ']);
        let layer: usize = split.next().unwrap().parse().unwrap();
        let depth: usize = split.skip(1).next().unwrap().parse().unwrap();
        num_layer = num_layer.max(layer);
        let scanner = Scanner::new(depth);
        scanner_map.insert(layer, scanner);
    }

    // Prepare
    let mut cache: Vec<HashSet<usize>> = Vec::new();

    // Solve
    'outer: loop {
        for current_position in 0..=num_layer {
            let current_time = delay + current_position;
            let hit_state: &HashSet<usize> =
                get_current_hit_state(current_time, &mut cache, &mut scanner_map);

            if hit_state.contains(&current_position) {
                delay += 1;
                continue 'outer;
            }
        }
        break 'outer;
    }

    // Result
    println!("Result of part 2 is {}", delay);
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
