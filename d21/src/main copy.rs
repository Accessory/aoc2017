use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

use utils::get_input_path;
use utils::grid::Grid;

#[derive(Debug)]
enum RuleType {
    TwoXTwo,
    ThreeXThree,
}

impl From<usize> for RuleType {
    fn from(value: usize) -> Self {
        match value {
            2 => Self::TwoXTwo,
            3 => Self::ThreeXThree,
            _ => panic!("Should not be here"),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rule {
    rule_type: RuleType,
    active_ones: usize,
    left: Grid<u8>,
    right: Grid<u8>,
}

fn insert_if_nessessary(
    data: Vec<Vec<u8>>,
    right: Vec<Vec<u8>>,
    active_ones: usize,
    rule_map: &mut HashMap<u64, Rule>,
) {
    let data_hash = create_hash(&data);
    if !rule_map.contains_key(&data_hash) {
        rule_map.insert(
            data_hash,
            Rule {
                rule_type: data.len().into(),
                active_ones: active_ones,
                left: Grid { data },
                right: Grid { data: right },
            },
        );
    }
}

fn create_hash(value: &Vec<Vec<u8>>) -> u64 {
    let mut default_hasher = DefaultHasher::new();
    value.hash(&mut default_hasher);
    default_hasher.finish()
}

fn symmetric(data: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    if data.len() == 2 {
        vec![vec![data[0][0], data[1][0]], vec![data[0][1], data[1][1]]]
    } else {
        vec![
            vec![data[0][0], data[1][0], data[2][0]],
            vec![data[0][1], data[1][1], data[2][1]],
            vec![data[0][2], data[1][2], data[2][2]],
        ]
    }
}

fn flip(data: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    if data.len() == 2 {
        vec![vec![data[1][0], data[1][1]], vec![data[0][0], data[0][1]]]
    } else {
        vec![
            vec![data[2][0], data[2][1], data[2][2]],
            vec![data[1][0], data[1][1], data[1][2]],
            vec![data[0][0], data[0][1], data[0][2]],
        ]
    }
}

fn create_new_grid(grid: &Grid<u8>, rule_map: &HashMap<u64, Rule>) -> Grid<u8> {
    if grid.get_max_y() % 2 == 0 {
        create_new_grid_with_sub_grid(grid, rule_map, 2)
    } else {
        create_new_grid_with_sub_grid(grid, rule_map, 3)
    }
}

fn create_new_grid_with_sub_grid(
    grid: &Grid<u8>,
    rule_map: &HashMap<u64, Rule>,
    sub_grid_size: usize,
) -> Grid<u8> {
    let max = grid.get_max_y() / sub_grid_size;

    let new_size = grid.get_max_y() + max;

    let mut rtn: Grid<u8> = Grid {
        data: vec![vec![0; new_size]; new_size],
    };

    for by in 0..max {
        for bx in 0..max {
            let nx = bx * sub_grid_size;
            let ny = by * sub_grid_size;

            let sub_grid = grid.create_sub_grid(nx, ny, sub_grid_size, sub_grid_size);
            let current_hash = create_hash(&sub_grid.data);
            let rule = rule_map.get(&current_hash).expect("Could not find a Rule");

            for y in 0..rule.right.get_max_y() {
                for x in 0..rule.right.get_max_y() {
                    let to_x = x + nx + bx;
                    let to_y = y + ny + by;

                    rtn.set(to_x, to_y, *rule.right.get(x, y).unwrap());
                }
            }
        }
    }
    // print_grid(&rtn);
    rtn
}

#[allow(dead_code)]
fn print_grid(rtn: &Grid<u8>) {
    for row in &rtn.data {
        for column in row {
            print!("{}", (*column) as char);
        }
        println!();
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut grid: Grid<u8> = Grid {
        data: vec![".#.".into(), "..#".into(), "###".into()],
    };

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut rule_map: HashMap<u64, Rule> = HashMap::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split(" => ");
        let left: Vec<Vec<u8>> = split
            .next()
            .unwrap()
            .split('/')
            .into_iter()
            .map(|i| i.as_bytes().iter().map(|i| *i).collect::<Vec<u8>>())
            .collect();
        let right: Vec<Vec<u8>> = split
            .next()
            .unwrap()
            .split('/')
            .into_iter()
            .map(|i| i.as_bytes().iter().map(|i| *i).collect::<Vec<u8>>())
            .collect();

        let active_ones = left.iter().flatten().filter(|i| *i == &b'#').count();

        let v1 = symmetric(&left);
        let v2 = flip(&v1);
        let v3 = symmetric(&v2);
        let v4 = flip(&v3);
        let v5 = symmetric(&v4);
        let v6 = flip(&v5);
        let v7 = symmetric(&v6);

        insert_if_nessessary(left, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v1, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v2, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v3, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v4, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v5, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v6, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v7, right, active_ones, &mut rule_map);
    });

    #[cfg(test)]
    for _ in 0..2 {
        grid = create_new_grid(&grid, &rule_map);
    }

    #[cfg(not(test))]
    for _ in 0..5 {
        grid = create_new_grid(&grid, &rule_map);
    }

    // print_grid(&grid);
    // Solve
    let result = grid.count_for(&b'#');
    // Result
    println!("Result of part 1 is {result}");
}

fn run2(input_file: &str) {
    // Preamble
    let mut grid: Grid<u8> = Grid {
        data: vec![".#.".into(), "..#".into(), "###".into()],
    };

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut rule_map: HashMap<u64, Rule> = HashMap::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split(" => ");
        let left: Vec<Vec<u8>> = split
            .next()
            .unwrap()
            .split('/')
            .into_iter()
            .map(|i| i.as_bytes().iter().map(|i| *i).collect::<Vec<u8>>())
            .collect();
        let right: Vec<Vec<u8>> = split
            .next()
            .unwrap()
            .split('/')
            .into_iter()
            .map(|i| i.as_bytes().iter().map(|i| *i).collect::<Vec<u8>>())
            .collect();

        let active_ones = left.iter().flatten().filter(|i| *i == &b'#').count();

        let v1 = symmetric(&left);
        let v2 = flip(&v1);
        let v3 = symmetric(&v2);
        let v4 = flip(&v3);
        let v5 = symmetric(&v4);
        let v6 = flip(&v5);
        let v7 = symmetric(&v6);

        insert_if_nessessary(left, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v1, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v2, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v3, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v4, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v5, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v6, right.clone(), active_ones, &mut rule_map);
        insert_if_nessessary(v7, right, active_ones, &mut rule_map);
    });

    #[cfg(test)]
    for _ in 0..2 {
        grid = create_new_grid(&grid, &rule_map);
    }

    #[cfg(not(test))]
    for _ in 0..18 {
        grid = create_new_grid(&grid, &rule_map);
    }

    // print_grid(&grid);
    // Solve
    let result = grid.count_for(&b'#');
    // Result
    println!("Result of part 2 is {result}");
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
