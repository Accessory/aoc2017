use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, Clone, Copy)]
struct Pair {
    pub v1: usize,
    pub v2: usize,
}

impl Pair {
    pub fn has_value(&self, value: usize) -> bool {
        self.v1 == value || self.v2 == value
    }

    fn has_zero(&self) -> bool {
        self.v1 == 0 || self.v2 == 0
    }

    fn other_value(&self, value: usize) -> usize {
        if self.v1 == value {
            self.v2
        } else {
            self.v1
        }
    }

    fn sum(&self) -> usize {
        self.v1 + self.v2
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct SearchContext {
    next: usize,
    bridge: u64,
}
impl SearchContext {
    fn sum(&self, pairs: &[Pair]) -> usize {
        let mut rtn = 0;
        for i in 0..pairs.len() {
            if 1 << i & self.bridge != 0 {
                rtn += pairs[i].sum();
            }
        }
        rtn
    }
}

fn sum_bridges(bridge: u64, pairs: &[Pair]) -> usize {
    let mut rtn = 0;
    for i in 0..pairs.len() {
        if 1 << i & bridge != 0 {
            rtn += pairs[i].sum();
        }
    }
    rtn
}

fn run(input_file: &str) {
    // Preamble

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let pairs: Vec<Pair> = reader
        .lines()
        .map(|i| {
            let is = i.unwrap();
            let mut split = is.trim().split('/');
            let v1 = split.next().unwrap().parse().unwrap();
            let v2 = split.next().unwrap().parse().unwrap();
            Pair { v1, v2 }
        })
        .collect();

    let mut to_calc: HashSet<u64> = HashSet::new();

    // Solve
    for start_idx in 0..pairs.len() {
        if !pairs[start_idx].has_zero() {
            continue;
        }

        let start = SearchContext {
            bridge: 1 << start_idx,
            next: pairs[start_idx].other_value(0),
        };

        let mut queue = vec![start];

        while !queue.is_empty() {
            let item = queue.pop().unwrap();

            for (c_idx, pair) in pairs.iter().enumerate() {
                let shift_idx = 1 << c_idx;
                if shift_idx & item.bridge != 0 {
                    continue;
                }

                let mut no_new_value = true;

                if pair.has_value(item.next) {
                    no_new_value = true;
                    let sc = SearchContext {
                        next: pair.other_value(item.next),
                        bridge: item.bridge + shift_idx,
                    };
                    queue.push(sc);
                }

                if no_new_value {
                    to_calc.insert(item.bridge);
                }
            }
        }
    }

    // Result
    let mut result = 0;

    for calc in to_calc {
        result = result.max(sum_bridges(calc, &pairs));
    }

    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let pairs: Vec<Pair> = reader
        .lines()
        .map(|i| {
            let is = i.unwrap();
            let mut split = is.trim().split('/');
            let v1 = split.next().unwrap().parse().unwrap();
            let v2 = split.next().unwrap().parse().unwrap();
            Pair { v1, v2 }
        })
        .collect();

    let mut to_calc = Vec::new();

    // Solve
    for start_idx in 0..pairs.len() {
        if !pairs[start_idx].has_zero() {
            continue;
        }

        let start = SearchContext {
            bridge: 1 << start_idx,
            next: pairs[start_idx].other_value(0),
        };

        let mut queue = vec![start];

        while !queue.is_empty() {
            let item = queue.pop().unwrap();

            for (c_idx, pair) in pairs.iter().enumerate() {
                let shift_idx = 1 << c_idx;
                if shift_idx & item.bridge != 0 {
                    continue;
                }

                let mut no_new_value = true;

                if pair.has_value(item.next) {
                    no_new_value = true;
                    let sc = SearchContext {
                        next: pair.other_value(item.next),
                        bridge: item.bridge + shift_idx,
                    };
                    queue.push(sc);
                }

                if no_new_value {
                    to_calc.push(item);
                }
            }
        }
    }

    // Result
    let mut result = 0;
    let mut length = 0;

    for calc in to_calc {
        let ones = calc.bridge.count_ones();
        if length == ones {
            result = result.max(calc.sum(&pairs));
        }
        if length < ones {
            result = calc.sum(&pairs);
            length = ones;
        }
    }

    println!("Result of part 2 is {}", result);
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
