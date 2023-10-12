use std::fs;

use utils::get_input_path;

fn reverse(current_position: usize, length: u8, list: &mut [u8]) {
    let end = (current_position + length as usize - 1) % list.len();
    let swaps = length as usize / 2;

    for i in 0..swaps {
        let start: usize = (i + current_position) % list.len();
        let end: usize = (list.len() + end - i) % list.len();
        list.swap(start, end);
    }
}

fn xor_list(list: &[u8]) -> u8 {
    let mut rtn: u8 = 0;
    for i in list {
        rtn ^= i;
    }
    rtn
}

fn dense_hashlist(list: &[u8]) -> [u8; 16] {
    let mut rtn = [0; 16];

    for i in 0..16 {
        let slice_start = i * 16;
        let slice_end = slice_start + 16;
        let to_rtn = xor_list(&list[slice_start..slice_end]);
        rtn[i] = to_rtn;
    }

    rtn
}

fn run(input_file: &str) {
    // Preamble
    let mut current_position = 0;
    let mut skip_lenght = 0;

    #[cfg(test)]
    let mut list: Vec<u8> = Vec::from_iter(0..=4);

    #[cfg(not(test))]
    let mut list: Vec<u8> = Vec::from_iter(0..=255);
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let lengths: Vec<u8> = line.split(',').map(|f| f.parse().unwrap()).collect();
    // Solve

    for length in lengths {
        reverse(current_position, length, &mut list);
        current_position = (current_position + skip_lenght + length as usize) % list.len();
        skip_lenght += 1;
    }

    // Result
    let result: usize = list[0] as usize * list[1] as usize;
    println!("Result is {}", result);
}

#[allow(unused_variables)]
fn run2(input_file: &str) {
    // Preamble
    let mut current_position: usize = 0;
    let mut skip_lenght: usize = 0;

    const RAW: [u8; 5] = [17, 31, 73, 47, 23];
    let mut list: Vec<u8> = Vec::from_iter(0..=255);

    // Parse
    #[cfg(test)]
    let line = "1,2,3";

    #[cfg(not(test))]
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();

    let mut lengths = Vec::from(line.as_bytes());
    lengths.extend(&RAW);

    for _ in 0..64 {
        for length in lengths.iter() {
            reverse(current_position, *length as u8, &mut list);
            current_position =
                (current_position + skip_lenght + *length as usize) % list.len() as usize;
            skip_lenght += 1;
        }
    }

    let result = dense_hashlist(&list);

    print!("Result of part 2 is ");
    result.iter().for_each(|b| {print!("{b:02x}")});
    println!();
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
