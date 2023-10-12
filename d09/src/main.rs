use std::fs;

use utils::get_input_path;

#[derive(PartialEq, Eq)]
enum ParserState {
    Start,
    Group,
    Garbage,
}

impl ParserState {
    pub fn looking_for(&self) -> Vec<char> {
        match self {
            ParserState::Group => vec!['{', '}', '<'],
            ParserState::Garbage => vec!['!', '>'],
            ParserState::Start => vec!['{'],
        }
    }
}

struct ParserContext {
    state: ParserState,
    next_score: usize,
}

enum ParserAction {
    GroupOpen,
    GroupClose,
    GarbageOpen,
    GarbageClose,
    Esquape,
    Skip,
}

impl ParserAction {
    fn to_action(value: char) -> Self {
        match value {
            '{' => ParserAction::GroupOpen,
            '}' => ParserAction::GroupClose,
            '<' => ParserAction::GarbageOpen,
            '>' => ParserAction::GarbageClose,
            '!' => ParserAction::Esquape,
            _ => ParserAction::Skip,
        }
    }
}

fn calculate_score(line: &str) -> usize {
    let mut score = 0;
    let mut context: ParserContext = ParserContext {
        state: ParserState::Start,
        next_score: 0,
    };

    let mut skip = false;

    for c in line.chars() {
        if skip {
            skip = false;
            continue;
        }

        if !context.state.looking_for().contains(&c) {
            continue;
        }

        let action: ParserAction = ParserAction::to_action(c);

        match action {
            ParserAction::GroupOpen => {
                context.state = ParserState::Group;
                context.next_score += 1;
            }
            ParserAction::GroupClose => {
                score += context.next_score;
                context.next_score -= 1;
                context.state = ParserState::Group;
            }
            ParserAction::GarbageOpen => {
                context.state = ParserState::Garbage;
            }
            ParserAction::GarbageClose => {
                context.state = ParserState::Group;
            }
            ParserAction::Esquape => {
                skip = true;
            }
            ParserAction::Skip => {}
        };
    }
    score
}

fn calculate_score2(line: &str) -> usize {
    let mut score = 0;
    let mut context: ParserContext = ParserContext {
        state: ParserState::Start,
        next_score: 0,
    };

    let mut skip = false;

    for c in line.chars() {
        if skip {
            skip = false;
            continue;
        }

        if !context.state.looking_for().contains(&c) {
            if context.state == ParserState::Garbage {
                score += 1;
            }
            continue;
        }

        let action: ParserAction = ParserAction::to_action(c);

        match action {
            ParserAction::GroupOpen => {
                context.state = ParserState::Group;
            }
            ParserAction::GroupClose => {
                context.state = ParserState::Group;
            }
            ParserAction::GarbageOpen => {
                context.state = ParserState::Garbage;
            }
            ParserAction::GarbageClose => {
                context.state = ParserState::Group;
            }
            ParserAction::Esquape => {
                skip = true;
            }
            ParserAction::Skip => {}
        };
    }
    score
}

fn run(input_file: &str) {
    // Preamble
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    let result = calculate_score(&line);

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
        // Preamble
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    let result = calculate_score2(&line);

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
