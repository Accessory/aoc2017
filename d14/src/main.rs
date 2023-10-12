use std::fs;

use utils::{
    get_input_path, map::Map, point::MapPoint,
};

// fn print_grid(grid: &[u128]) {
//     for row in grid {
//         println!("{:0128b}", row);
//     }
// }

// fn print_map(map: &Map) {
//     for (i, item) in map.data.iter().enumerate() {
//         print!("{}", item % 10);
//         if (i + 1) % map.max_x as usize == 0 {
//             println!()
//         }
//     }
// }

fn is_hit(x: i64, y: i64, grid: &[u128]) -> bool {
    (grid[y as usize] & 1 << (127 - x)) != 0
}

fn create_grid(line: &str) -> Vec<u128> {
    let mut grid: Vec<u128> = Vec::with_capacity(128);

    for i in 0..128 {
        let to_hash_string = format!("{}-{}", &line, &i);
        let hash = create_hash(&to_hash_string);
        let mut to_grid: u128 = 0;

        to_grid += (hash[15] as u128) << 0;
        to_grid += (hash[14] as u128) << 8;
        to_grid += (hash[13] as u128) << 16;
        to_grid += (hash[12] as u128) << 24;
        to_grid += (hash[11] as u128) << 32;
        to_grid += (hash[10] as u128) << 40;
        to_grid += (hash[9] as u128) << 48;
        to_grid += (hash[8] as u128) << 56;
        to_grid += (hash[7] as u128) << 64;
        to_grid += (hash[6] as u128) << 72;
        to_grid += (hash[5] as u128) << 80;
        to_grid += (hash[4] as u128) << 88;
        to_grid += (hash[3] as u128) << 96;
        to_grid += (hash[2] as u128) << 104;
        to_grid += (hash[1] as u128) << 112;
        to_grid += (hash[0] as u128) << 120;

        grid.push(to_grid);
    }

    grid
}

fn create_hash(to_hash_string: &String) -> [u8; 16] {
    const RAW: [u8; 5] = [17, 31, 73, 47, 23];
    let mut current_position = 0;
    let mut skip_lenght = 0;
    let mut list: Vec<u8> = Vec::from_iter(0..=255);
    let mut length = Vec::from_iter(to_hash_string.bytes());
    length.extend(RAW);
    for _ in 0..64 {
        for byte in length.iter() {
            reverse(current_position, *byte as u8, &mut list);
            current_position =
                (current_position + skip_lenght + *byte as usize) % list.len() as usize;
            skip_lenght += 1;
        }
    }

    dense_hashlist(&list)
}

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
    // println!("{list:?}");
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
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    // Solve
    let grid: Vec<u128> = create_grid(&line);

    // print_grid(&grid);
    // Result
    let result: u32 = grid.iter().map(|i| i.count_ones()).sum();
    println!("Result of part 1 is {result}");
}

fn run2(input_file: &str) {
    // Preamble
    let mut map: Map = Map::new(128, 128);

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Prepare
    let grid: Vec<u128> = create_grid(&line);
    let mut current_group: i64 = 0;

    // Solve
    for i in 0..128 * 128 {
        let x = i % 128;
        let y = i / 128;

        let point = MapPoint { x, y };

        if !is_hit(x, y, &grid) || map.get(x, y) != 0 {
            continue;
        }

        current_group += 1;
        map.set_at_point(&point, current_group);

        let mut neigbors = point.generate_non_diagonal_neigbors();

        while let Some(neigbor) = neigbors.pop() {
            if map.is_point_in_map(neigbor)
                && is_hit(neigbor.x, neigbor.y, &grid)
                && map.get_from_point(&neigbor) == 0
            {
                map.set_at_point(&neigbor, current_group);
                neigbors.extend(neigbor.generate_non_diagonal_neigbors());
            }
        }
    }

    // print_map(&map);

    println!("Result of part 2 is {current_group}");
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
