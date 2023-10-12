use std::fs;

use utils::get_input_path;

#[derive(Debug)]
enum Instruction {
    Spin { v1: usize },
    Exchange { v1: usize, v2: usize },
    Partner { v1: char, v2: char },
}

impl Instruction {
    pub fn new(value: &str) -> Self {
        let i_type = value.chars().next().unwrap();

        match i_type {
            's' => Instruction::Spin {
                v1: value[1..].parse().unwrap(),
            },
            'x' => {
                let mut split = value[1..].split('/');
                Instruction::Exchange {
                    v1: split.next().unwrap().parse().unwrap(),
                    v2: split.next().unwrap().parse().unwrap(),
                }
            }
            'p' => {
                let mut split = value[1..].split('/');
                Instruction::Partner {
                    v1: split.next().unwrap().chars().next().unwrap(),
                    v2: split.next().unwrap().chars().next().unwrap(),
                }
            }
            _ => panic!("Should not be here - Value: {i_type}"),
        }
    }

    pub fn work_on(&self, line: &mut [char]) {
        match self {
            Instruction::Spin { v1 } => {
                line.rotate_right(*v1);
            }
            Instruction::Exchange { v1, v2 } => {
                line.swap(*v1, *v2);
            }
            Instruction::Partner { v1, v2 } => {
                let mut p1 = usize::MAX;
                let mut p2 = usize::MAX;
                for (i, char) in line.iter().enumerate() {
                    if char == v1 {
                        p1 = i;
                    }
                    if char == v2 {
                        p2 = i;
                    }
                    if p1 != usize::MAX && p2 != usize::MAX {
                        break;
                    }
                }
                line.swap(p1, p2);
            }
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    #[cfg(not(test))]
    let mut order = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];

    #[cfg(test)]
    let mut order: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let instructions: Vec<Instruction> = line.split(",").map(|f| Instruction::new(f)).collect();

    // Solve
    for instruction in instructions {
        instruction.work_on(&mut order);
    }

    // Result
    print!("Result of part 1 is: ");
    order.iter().for_each(|o| print!("{o}"));
    println!()
}

fn run2(input_file: &str) {
    // Preamble
    const BILLION: usize = 1_000_000_000;
    #[cfg(not(test))]
    let mut order = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];

    #[cfg(test)]
    let mut order: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    let mut hashs: Vec<String> = Vec::new();

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let instructions: Vec<Instruction> = line.split(",").map(|f| Instruction::new(f)).collect();

    // Solve
    let mut i: usize = 0;
    let order_string = String::from_iter(order);
    hashs.push(order_string);
    while i < BILLION {
        i += 1;
        // Dance
        for instruction in instructions.iter() {
            instruction.work_on(&mut order);
        }

        // Hash
        let order_string = String::from_iter(order);
        if hashs.contains(&order_string) {
            break;
        }

        hashs.push(order_string);
    }

    // Result
    let result = BILLION % hashs.len();
    print!("Result of part 2 is: {}", hashs.get(result).unwrap());
    println!()
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
