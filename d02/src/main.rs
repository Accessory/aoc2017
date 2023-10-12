use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut rows = Vec::new();
    let mut result = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let row:Vec<usize> = line.split(['\t',' ']).map(|f| f.parse::<usize>().unwrap()).collect();
        rows.push(row);
    }

    // Solve
    for row in rows {
        let mut min = usize::MAX;
        let mut max = usize::MIN;

        for i in row {
            min = i.min(min);
            max = i.max(max);
        }

        let to_add = max - min;
        result += to_add;
    }


    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
        // Preamble
        let mut rows = Vec::new();
        let mut result = 0;
    
        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
            let row:Vec<usize> = line.split(['\t',' ']).map(|f| f.parse::<usize>().unwrap()).collect();
            rows.push(row);
        }
    
        // Solve
        for mut row in rows {
            row.sort_unstable_by(|l,r| r.cmp(l));
            
            for p1 in 0..row.len(){
                for p2 in (p1+1..row.len()).rev(){
                    let z1 = row.get(p1).unwrap();
                    let z2 = row.get(p2).unwrap();
                    if z1 % z2 == 0 {
                        let to_add = z1 / z2;
                        result += to_add;
                        break;
                    }
                }
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
