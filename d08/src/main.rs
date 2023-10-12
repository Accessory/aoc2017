use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug)]
enum Operation {
    Inc,
    Dec,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "inc" => Operation::Inc,
            "dec" => Operation::Dec,
            _ => panic!("Wront input {value}"),
        }
    }
}

#[derive(Debug)]
enum Condition {
    Geater,
    Lesser,
    GreaterEquils,
    LesserEquils,
    Equils,
    EquilsNot,
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        match value {
            "<" => Condition::Lesser,
            ">" => Condition::Geater,
            "<=" => Condition::LesserEquils,
            ">=" => Condition::GreaterEquils,
            "==" => Condition::Equils,
            "!=" => Condition::EquilsNot,
            _ => panic!("Wrong input {value}"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    register: String,
    operation: Operation,
    operation_value: i64,
    condition_register: String,
    condition: Condition,
    condition_value: i64,
}
impl Instruction {
    fn execute(&self, register_map: &mut HashMap<String, i64>) {
        if self.check_condition(register_map) {
            match self.operation {
                Operation::Inc => {
                    *register_map.get_mut(&self.register).unwrap() += self.operation_value;
                }
                Operation::Dec => {
                    *register_map.get_mut(&self.register).unwrap() -= self.operation_value;
                }
            }
        }
    }

    fn check_condition(&self, register_map: &mut HashMap<String, i64>) -> bool {
        let conditional_variable_value = *register_map.get(&self.condition_register).unwrap();

        match self.condition {
            Condition::Geater => conditional_variable_value > self.condition_value,
            Condition::Lesser => conditional_variable_value < self.condition_value,
            Condition::GreaterEquils => conditional_variable_value >= self.condition_value,
            Condition::LesserEquils => conditional_variable_value <= self.condition_value,
            Condition::Equils => conditional_variable_value == self.condition_value,
            Condition::EquilsNot => conditional_variable_value != self.condition_value,
        }
    }

    fn execute_with_callback(&self, register_map: &mut HashMap<String, i64>, callback: impl FnOnce(i64)) {
        if self.check_condition(register_map) {
            match self.operation {
                Operation::Inc => {
                    *register_map.get_mut(&self.register).unwrap() += self.operation_value;
                }
                Operation::Dec => {
                    *register_map.get_mut(&self.register).unwrap() -= self.operation_value;
                }
            };
            callback(*register_map.get(&self.register).unwrap());
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split(" ").collect();
        let register: String = split.get(0).unwrap().to_string();
        let operation: Operation = (*split.get(1).unwrap()).into();
        let operation_value: i64 = split.get(2).unwrap().parse().unwrap();
        let condition_register: String = split.get(4).unwrap().to_string();
        let condition: Condition = (*split.get(5).unwrap()).into();
        let condition_value: i64 = split.get(6).unwrap().parse().unwrap();

        Self {
            register,
            operation,
            operation_value,
            condition_register,
            condition,
            condition_value,
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut register_map: HashMap<String, i64> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let ls = line.unwrap();
        let line = ls.trim();
        let instruction: Instruction = line.into();

        // Prepare
        if !register_map.contains_key(&instruction.register) {
            register_map.insert(instruction.register.clone(), 0);
        }

        instructions.push(instruction);
    }

    // Solve
    for instruction in instructions {
        instruction.execute(&mut register_map);
    }

    // Result
    let result = register_map.iter().map(|f| f.1).max().unwrap();
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut register_map: HashMap<String, i64> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let ls = line.unwrap();
        let line = ls.trim();
        let instruction: Instruction = line.into();

        // Prepare
        if !register_map.contains_key(&instruction.register) {
            register_map.insert(instruction.register.clone(), 0);
        }

        instructions.push(instruction);
    }

    // Solve
    let mut result = i64::MIN;
    for instruction in instructions {
        instruction.execute_with_callback(&mut register_map, |value: i64| {
            result = result.max(value);
        });
    }

    // Result

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
