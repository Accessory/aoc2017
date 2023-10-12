use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;
use utils::grid::Grid;
use utils::grid_direction::GridDirection;
use utils::grid_point::GridPoint;

fn check_next_move(
    point: &GridPoint,
    direction: &GridDirection,
    grid: &Grid<char>,
    max_x: usize,
    max_y: usize,
) -> Option<GridDirection> {
    let neighbors = point.generate_non_diagonal_neigbors_with_check(max_x, max_y);

    if !can_go_on(&neighbors, grid, direction.get_int_char()) {
        return None;
    }

    if grid.get_from_point(point).is_some_and(|i| *i != '+') {
        return Some(*direction);
    } else {
        for direction in GridDirection::get_directions() {
            let point_to_check = point.next_by_direction_with_check(&direction, max_x, max_y);
            if point_to_check.is_some_and(|ptc| {
                grid.get_from_point(&ptc)
                    .is_some_and(|i| ['-', '|'].contains(i) || i.is_alphabetic())
            }) {
                return Some(direction);
            }
        }
    }
    None
}

fn can_go_on(neighbors: &[GridPoint], grid: &Grid<char>, exclude: char) -> bool {
    for gp in neighbors {
        if grid
            .get_from_point(gp)
            .is_some_and(|i| *i != ' ' && *i != exclude)
        {
            return true;
        }
    }
    false
}

fn run(input_file: &str) {
    // Preamble
    let mut grid: Grid<char> = Grid::default();
    let mut result = String::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().to_string();
        grid.data.push(line.chars().collect());
    }

    // Prepare
    let mut current_position = GridPoint::default();
    let mut current_direction = GridDirection::Down;

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    while grid.get_from_point(&current_position).unwrap() == &' ' {
        current_position.move_right();
    }

    // Solve
    loop {
        grid.set_from_point(&current_position, current_direction.get_int_char());
        current_position.move_direction(&current_direction);
        // println!(
        //     "Position: x: {}, y: {}, Symbol: {}",
        //     current_position.x,
        //     current_position.y,
        //     grid.get_from_point(&current_position).unwrap()
        // );
        let c = grid.get_from_point(&current_position).unwrap();
        if c.is_alphabetic() {
            result.push(*c);
        }

        if let Some(next_direction) =
            check_next_move(&current_position, &current_direction, &grid, max_x, max_y)
        {
            current_direction = next_direction;
        } else {
            break;
        }
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
     // Preamble
     let mut grid: Grid<char> = Grid::default();
     let mut result:usize = 1;
 
     // Parse
     let file = File::open(input_file).unwrap();
     let reader = BufReader::new(file);
 
     for line in reader.lines() {
         let line = line.unwrap().to_string();
         grid.data.push(line.chars().collect());
     }
 
     // Prepare
     let mut current_position = GridPoint::default();
     let mut current_direction = GridDirection::Down;
 
     let max_x = grid.get_max_x();
     let max_y = grid.get_max_y();
 
     while grid.get_from_point(&current_position).unwrap() == &' ' {
         current_position.move_right();
     }
 
     // Solve
     loop {
         grid.set_from_point(&current_position, current_direction.get_int_char());
         current_position.move_direction(&current_direction);
         // println!(
         //     "Position: x: {}, y: {}, Symbol: {}",
         //     current_position.x,
         //     current_position.y,
         //     grid.get_from_point(&current_position).unwrap()
         // );
        //  let c = grid.get_from_point(&current_position).unwrap();
        //  if c.is_alphabetic() {
        //      result.push(*c);
        //  }

        result += 1;
 
         if let Some(next_direction) =
             check_next_move(&current_position, &current_direction, &grid, max_x, max_y)
         {
             current_direction = next_direction;
         } else {
             break;
         }
     }
 
     // Result
     println!("Result of part 2 is {result}");
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
