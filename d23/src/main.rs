use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, Copy, Clone)]
enum Parameter {
    Number(i64),
    Register(usize),
}
impl Parameter {
    fn get_value(&self, context: &Context) -> i64 {
        match self {
            Parameter::Number(value) => *value,
            Parameter::Register(register) => context.registers[*register],
        }
    }
    fn set_value(&self, value: i64, context: &mut Context) {
        match self {
            Parameter::Number(_) => panic!("Should not be here"),
            Parameter::Register(register) => {
                context.registers[*register] = value;
            }
        };
    }
}

impl From<&str> for Parameter {
    fn from(value: &str) -> Self {
        match value.parse() {
            Ok(value) => Self::Number(value),
            Err(_) => Self::Register(((value.chars().next().unwrap() as u8) - b'a') as usize),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Set(Parameter, Parameter),
    Jnz(Parameter, Parameter),
    Sub(Parameter, Parameter),
    Mul(Parameter, Parameter),
}

struct Context {
    registers: [i64; 8],
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let mut split = value.split(' ');

        let instruction: &str = split.next().unwrap();
        let parameter1: Parameter = split.next().unwrap().into();
        let parameter2: Parameter = split.next().unwrap().into();

        match instruction {
            "set" => Instruction::Set(parameter1, parameter2),
            "jnz" => Instruction::Jnz(parameter1, parameter2),
            "sub" => Instruction::Sub(parameter1, parameter2),
            "mul" => Instruction::Mul(parameter1, parameter2),
            _ => panic!("Should not be here"),
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut context: Context = Context { registers: [0; 8] };
    let mut called_mul: usize = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let instractions: Vec<Instruction> = reader.lines().map(|i| i.unwrap().into()).collect();

    // Solve
    let mut idx: usize = 0;
    loop {
        if let Some(instruction) = instractions.get(idx) {
            match instruction {
                Instruction::Set(p1, p2) => {
                    let value = p2.get_value(&context);
                    p1.set_value(value, &mut context);
                }
                Instruction::Jnz(p1, p2) => {
                    let value = p1.get_value(&context);
                    if value != 0 {
                        let value_2 = p2.get_value(&context);
                        idx = (idx as i64 + value_2) as usize;
                        continue;
                    }
                }
                Instruction::Sub(p1, p2) => {
                    let value_1 = p1.get_value(&context);
                    let value_2 = p2.get_value(&context);
                    p1.set_value(value_1 - value_2, &mut context);
                }
                Instruction::Mul(p1, p2) => {
                    let value_1 = p1.get_value(&context);
                    let value_2 = p2.get_value(&context);
                    p1.set_value(value_1 * value_2, &mut context);
                    called_mul += 1;
                }
            };
            idx += 1;
        } else {
            break;
        }
    }

    // Result
    println!("Result of part 1 is {}", called_mul);
}

fn run2(input_file: &str) {
    // Preamble
    let mut context: Context = Context {
        registers: [1, 0, 0, 0, 0, 0, 0, 0],
    };

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let instractions: Vec<Instruction> = reader.lines().map(|i| i.unwrap().into()).collect();

    // Solve
    let mut idx: usize = 0;
    loop {
        if let Some(instruction) = instractions.get(idx) {
            if idx < 11 {
                match instruction {
                    Instruction::Set(p1, p2) => {
                        let value = p2.get_value(&context);
                        p1.set_value(value, &mut context);
                    }
                    Instruction::Jnz(p1, p2) => {
                        let value = p1.get_value(&context);
                        if value != 0 {
                            let value_2 = p2.get_value(&context);
                            idx = (idx as i64 + value_2) as usize;
                            continue;
                        }
                    }
                    Instruction::Sub(p1, p2) => {
                        let value_1 = p1.get_value(&context);
                        let value_2 = p2.get_value(&context);
                        p1.set_value(value_1 - value_2, &mut context);
                    }
                    Instruction::Mul(p1, p2) => {
                        let value_1 = p1.get_value(&context);
                        let value_2 = p2.get_value(&context);
                        p1.set_value(value_1 * value_2, &mut context);
                    }
                };
                idx += 1;
            } else {
                let mut nonprimes = 0;
                let start = context.registers[1];
                let end = context.registers[2];
                for b in (start..=end).step_by(17) {
                    let br = (b as f64).powf(0.5) as i64;
                    for dx in 2..br {
                        if b % dx == 0 {
                            nonprimes += 1;
                            break;
                        }
                    }

                    context.registers[7] = nonprimes;
                    idx = usize::MAX;
                }
            }
        } else {
            break;
        }
    }

    // Result
    println!("Result of part 2 is {}", context.registers[7]);
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
