use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn has_anagram(line: &str) -> bool {
    let mut char_counter_set = HashSet::with_capacity(12);

    for word in line.split(' ') {
        let char_counter = CharCounter::new(word);
        if !char_counter_set.insert(char_counter) {
            return true;
        }
    }

    false
}

fn has_duplicate_words(line: &str) -> bool {
    let mut word_set = HashSet::with_capacity(12);

    for word in line.split(' ') {
        if !word_set.insert(word) {
            return true;
        }
    }

    false
}

#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct CharCounter {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    f: usize,
    g: usize,
    h: usize,
    i: usize,
    j: usize,
    k: usize,
    l: usize,
    m: usize,
    n: usize,
    o: usize,
    p: usize,
    q: usize,
    r: usize,
    s: usize,
    t: usize,
    u: usize,
    v: usize,
    w: usize,
    x: usize,
    y: usize,
    z: usize,
}
impl CharCounter {
    fn new(word: &str) -> Self {
        let mut rtn = Self::default();

        for c in word.chars() {
            rtn.add_char(c);
        }

        rtn
    }

    fn add_char(&mut self, c: char) {
        match c {
            'a' => self.a += 1,
            'b' => self.b += 1,
            'c' => self.c += 1,
            'd' => self.d += 1,
            'e' => self.e += 1,
            'f' => self.f += 1,
            'g' => self.g += 1,
            'h' => self.h += 1,
            'i' => self.i += 1,
            'j' => self.j += 1,
            'k' => self.k += 1,
            'l' => self.l += 1,
            'm' => self.m += 1,
            'n' => self.n += 1,
            'o' => self.o += 1,
            'p' => self.p += 1,
            'q' => self.q += 1,
            'r' => self.r += 1,
            's' => self.s += 1,
            't' => self.t += 1,
            'u' => self.u += 1,
            'v' => self.v += 1,
            'w' => self.w += 1,
            'x' => self.x += 1,
            'y' => self.y += 1,
            'z' => self.z += 1,
            _ => panic!("Should not be here with {c}"),
        };
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut result = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if !has_duplicate_words(&line) {
            result += 1;
        }
    }

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if !has_anagram(&line) {
            result += 1;
        }
    }

    // Result
    println!("Result is {}", result);
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
