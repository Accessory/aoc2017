use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, Clone)]
struct Context {
    position: i64,
    state: u8,
    tape: HashMap<i64, bool>,
    steps: usize,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct State {
    state: u8,
    zero_write: bool,
    zero_move: i8,
    zero_continue: u8,
    one_write: bool,
    one_move: i8,
    one_continue: u8,
}

fn parse_file(input_file: &str) -> (Context, Vec<State>) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let state = lines.next().unwrap().unwrap().chars().nth_back(1).unwrap() as u8 - b'A';

    let steps = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .nth_back(1)
        .unwrap()
        .parse()
        .unwrap();

    let mut states = Vec::new();

    while lines.next().is_some() {
        let in_state = lines.next().unwrap().unwrap().chars().nth_back(1).unwrap() as u8 - b'A';
        let _ = lines.next().unwrap();
        let zero_write: bool = lines.next().unwrap().unwrap().chars().nth_back(1).unwrap() == '1';
        let zero_move = match lines.next().unwrap().unwrap().split(' ').last() {
            Some(value) => {
                if value == "right." {
                    1
                } else {
                    -1
                }
            }
            None => panic!("Should not be here!"),
        };
        let zero_continue =
            lines.next().unwrap().unwrap().chars().nth_back(1).unwrap() as u8 - b'A';
        let _ = lines.next().unwrap();
        let one_write: bool = lines.next().unwrap().unwrap().chars().nth_back(1).unwrap() == '1';
        let one_move = match lines.next().unwrap().unwrap().split(' ').last() {
            Some(value) => {
                if value == "right." {
                    1
                } else {
                    -1
                }
            }
            None => panic!("Should not be here!"),
        };
        let one_continue = lines.next().unwrap().unwrap().chars().nth_back(1).unwrap() as u8 - b'A';

        states.push(State {
            state: in_state,
            zero_write,
            zero_move,
            zero_continue,
            one_write,
            one_move,
            one_continue,
        });
    }

    (
        Context {
            position: 0,
            state,
            tape: HashMap::new(),
            steps,
        },
        states,
    )
}

fn run(input_file: &str) {
    // Preamble
    // Parse
    let (mut context, states) = parse_file(input_file);

    for _ in 0..context.steps {
        let state = &states[context.state as usize];
        let value = context.tape.entry(context.position).or_insert(false);

        if *value {
            context.position += state.one_move as i64;
            *value = state.one_write;
            context.state = state.one_continue;
        } else {
            context.position += state.zero_move as i64;
            *value = state.zero_write;
            context.state = state.zero_continue;
        }
    }

    // Solve
    // Result
    let result = context.tape.values().filter(|i| **i).count();

    println!("Result is {}", result);
}

fn run2(_input_file: &str) {}

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
