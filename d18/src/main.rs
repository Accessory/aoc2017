use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, Clone, Copy)]
enum RegisterValue {
    Register(char),
    Value(i64),
}
impl RegisterValue {
    fn eval(&self, registers: &HashMap<char, i64>) -> i64 {
        match self {
            RegisterValue::Register(register) => *registers.get(register).unwrap(),
            RegisterValue::Value(value) => *value,
        }
    }
}

impl From<&str> for RegisterValue {
    fn from(value: &str) -> Self {
        let c = value.chars().next().unwrap();
        if c.is_digit(10) || c == '-' {
            RegisterValue::Value(value.parse().unwrap())
        } else {
            RegisterValue::Register(c)
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Snd {
        register: char,
    },
    Set {
        register: char,
        value: RegisterValue,
    },
    Add {
        register: char,
        value: RegisterValue,
    },
    Mul {
        register: char,
        value: RegisterValue,
    },
    Mod {
        register: char,
        value: RegisterValue,
    },
    Rcv {
        register: char,
    },
    Jgz {
        value_1: RegisterValue,
        value_2: RegisterValue,
    },
}
impl Instruction {
    fn execute(&self, registers: &mut HashMap<char, i64>) -> [i64; 2] {
        let mut next = 1;
        let mut rcv = 0;
        match self {
            Instruction::Snd { register } => self.snd(register, registers),
            Instruction::Set { register, value } => self.set(register, value, registers),
            Instruction::Add { register, value } => self.add(register, value, registers),
            Instruction::Mul { register, value } => self.mul(register, value, registers),
            Instruction::Mod { register, value } => self.modular(register, value, registers),
            Instruction::Rcv { register } => {
                rcv = self.rcv(register, registers);
            }
            Instruction::Jgz { value_1, value_2 } => {
                next = self.jgz(value_1, value_2, registers);
            }
        };

        [next, rcv]
    }

    fn execute2(
        &self,
        registers: &mut HashMap<char, i64>,
        recv_queue: &mut VecDeque<i64>,
        send_queue: &mut VecDeque<i64>,
        is_execution_0: bool,
    ) -> i64 {
        let mut next = 1;
        match self {
            Instruction::Snd { register } => {
                self.snd2(register, registers, send_queue, is_execution_0)
            }
            Instruction::Set { register, value } => self.set(register, value, registers),
            Instruction::Add { register, value } => self.add(register, value, registers),
            Instruction::Mul { register, value } => self.mul(register, value, registers),
            Instruction::Mod { register, value } => self.modular(register, value, registers),
            Instruction::Rcv { register } => next = self.rcv2(register, registers, recv_queue),
            Instruction::Jgz { value_1, value_2 } => next = self.jgz(value_1, value_2, registers),
        };
        next
    }

    fn snd(&self, register: &char, registers: &mut HashMap<char, i64>) {
        registers.insert('0', *registers.get(register).unwrap());
    }

    fn rcv(&self, register: &char, registers: &mut HashMap<char, i64>) -> i64 {
        *registers.get(register).unwrap()
    }

    fn set(&self, register: &char, value: &RegisterValue, registers: &mut HashMap<char, i64>) {
        let value = value.eval(registers);
        registers
            .entry(*register)
            .and_modify(|i| *i = value)
            .or_insert(value);
    }

    fn add(&self, register: &char, value: &RegisterValue, registers: &mut HashMap<char, i64>) {
        let value = value.eval(registers);
        registers
            .entry(*register)
            .and_modify(|i| *i = i.checked_add(value).unwrap())
            .or_insert(value);
    }

    fn mul(&self, register: &char, value: &RegisterValue, registers: &mut HashMap<char, i64>) {
        let value = value.eval(registers);
        registers
            .entry(*register)
            .and_modify(|i| *i = i.checked_mul(value).unwrap())
            .or_insert(0);
    }

    fn modular(&self, register: &char, value: &RegisterValue, registers: &mut HashMap<char, i64>) {
        let value = value.eval(registers);
        registers
            .entry(*register)
            .and_modify(|i| *i %= value)
            .or_insert(0);
    }

    fn jgz(
        &self,
        v1: &RegisterValue,
        v2: &RegisterValue,
        registers: &mut HashMap<char, i64>,
    ) -> i64 {
        let v1v = v1.eval(&registers);
        let v2v: i64 = v2.eval(&registers);
        if v1v > 0 {
            v2v
        } else {
            1
        }
    }

    fn snd2(
        &self,
        register: &char,
        registers: &mut HashMap<char, i64>,
        send_queue: &mut VecDeque<i64>,
        is_execution_0: bool,
    ) {
        if !is_execution_0 {
            registers.entry('0').and_modify(|i| *i += 1).or_insert(1);
        }

        if register.is_digit(10) {
            let digit = register.to_digit(10).unwrap() as i64;
            send_queue.push_back(digit);
        } else {
            let value = *registers.get(register).unwrap();
            send_queue.push_back(value);
        }
    }

    fn rcv2(
        &self,
        register: &char,
        registers: &mut HashMap<char, i64>,
        recv_queue: &mut VecDeque<i64>,
    ) -> i64 {
        if let Some(entry) = recv_queue.pop_front() {
            registers.insert(*register, entry);
            1
        } else {
            0
        }
    }
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let mut split = value.split(" ");

        match split.next().unwrap() {
            "snd" => Instruction::Snd {
                register: split.next().unwrap().chars().next().unwrap(),
            },
            "set" => Instruction::Set {
                register: split.next().unwrap().chars().next().unwrap(),
                value: split.next().unwrap().into(),
            },
            "add" => Instruction::Add {
                register: split.next().unwrap().chars().next().unwrap(),
                value: split.next().unwrap().into(),
            },
            "mul" => Instruction::Mul {
                register: split.next().unwrap().chars().next().unwrap(),
                value: split.next().unwrap().into(),
            },
            "mod" => Instruction::Mod {
                register: split.next().unwrap().chars().next().unwrap(),
                value: split.next().unwrap().into(),
            },
            "rcv" => Instruction::Rcv {
                register: split.next().unwrap().chars().next().unwrap(),
            },
            "jgz" => Instruction::Jgz {
                value_1: split.next().unwrap().into(),
                value_2: split.next().unwrap().into(),
            },
            _ => panic!("Should not be here!: {value}"),
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut registers: HashMap<char, i64> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader.lines().map(|l| l.unwrap().into()).collect();

    // Solve
    let mut current_positon: i64 = 0;
    let mut jmp;
    let mut rcv = 0;
    while rcv == 0 {
        let instuction = instructions.get(current_positon as usize).unwrap();
        [jmp, rcv] = instuction.execute(&mut registers);

        current_positon += jmp;
    }

    // Result
    println!("Result of part 1 is {}", registers.get(&'0').unwrap());
}

fn run2(input_file: &str) {
    // Preamble
    let mut registers_0: HashMap<char, i64> = HashMap::new();
    registers_0.insert('p', 0);

    let mut registers_1: HashMap<char, i64> = HashMap::new();
    registers_1.insert('p', 1);
    registers_1.insert('0', 0);

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader.lines().map(|l| l.unwrap().into()).collect();

    // Solve
    let mut is_execution_0 = true;
    let mut current_position_0: i64 = 0;
    let mut current_position_1: i64 = 0;
    let mut jmp;
    let mut deadlock = false;
    let mut queue_0: VecDeque<i64> = VecDeque::new();
    let mut queue_1: VecDeque<i64> = VecDeque::new();
    let mut first_change = true;
    while !deadlock {
        if is_execution_0 {
            let instruction = instructions.get(current_position_0 as usize).unwrap();
            jmp =
                instruction.execute2(&mut registers_0, &mut queue_0, &mut queue_1, is_execution_0);
            current_position_0 += jmp;
        } else {
            let instruction = instructions.get(current_position_1 as usize).unwrap();
            jmp =
                instruction.execute2(&mut registers_1, &mut queue_1, &mut queue_0, is_execution_0);
            current_position_1 += jmp;
        }

        if jmp == 0 {
            is_execution_0 = !is_execution_0;

            if first_change {
                first_change = false;
                continue;
            }

            deadlock = if is_execution_0 {
                queue_0.is_empty()
            } else {
                queue_1.is_empty()
            };
        }
    }

    // Result
    println!("Result of part 2 is {}", registers_1.get(&'0').unwrap());
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
    use utils::get_test_input_2_path;
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
        let input_path = get_test_input_2_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
