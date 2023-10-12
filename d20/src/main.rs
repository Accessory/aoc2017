use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;
use utils::get_input_path;
use utils::utils::manhatten_distance_3d_from_zero;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Particles {
    p_x: i64,
    p_y: i64,
    p_z: i64,
    v_x: i64,
    v_y: i64,
    v_z: i64,
    a_x: i64,
    a_y: i64,
    a_z: i64,
}
impl Particles {
    fn add_acceleration(&mut self) {
        self.v_x += self.a_x;
        self.v_y += self.a_y;
        self.v_z += self.a_z;
    }

    fn add_velocity(&mut self) {
        self.p_x += self.v_x;
        self.p_y += self.v_y;
        self.p_z += self.v_z;
    }

    fn tick(&mut self) {
        self.add_acceleration();
        self.add_velocity()
    }

    fn get_position(&self) -> (i64, i64, i64) {
        (self.p_x, self.p_y, self.p_z)
    }
}

impl std::fmt::Debug for Particles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
            self.p_x,
            self.p_y,
            self.p_z,
            self.v_x,
            self.v_y,
            self.v_z,
            self.a_x,
            self.a_y,
            self.a_z
        ))
    }
}

impl From<String> for Particles {
    fn from(value: String) -> Self {
        let re = Regex::new(r#"p=< ?(-?\d+), ?(-?\d+), ?(-?\d+)>, v=< ?(-?\d+), ?(-?\d+), ?(-?\d+)>, a=< ?(-?\d+), ?(-?\d+), ?(-?\d+)>"#).unwrap();
        if let Some(captures) = re.captures(&value) {
            Particles {
                p_x: captures[1].parse().unwrap(),
                p_y: captures[2].parse().unwrap(),
                p_z: captures[3].parse().unwrap(),
                v_x: captures[4].parse().unwrap(),
                v_y: captures[5].parse().unwrap(),
                v_z: captures[6].parse().unwrap(),
                a_x: captures[7].parse().unwrap(),
                a_y: captures[8].parse().unwrap(),
                a_z: captures[9].parse().unwrap(),
            }
        } else {
            panic!("Error could not parse String: {value}");
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut accelation_sums: Vec<i64> = Vec::new();
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let particles: Vec<Particles> = reader.lines().map(|i| i.unwrap().into()).collect();

    // dbg!(particles);
    for particle in particles.iter() {
        let accelation_sum =
            manhatten_distance_3d_from_zero(particle.a_x, particle.a_y, particle.a_z);
        accelation_sums.push(accelation_sum)
    }

    // Solve
    let (result, s) = accelation_sums
        .iter()
        .enumerate()
        .min_by_key(|i| i.1)
        .unwrap();

    // Result
    println!(
        "Result of part 1 is {} with acceleration sum {}. Particel: {:?}",
        &result,
        s,
        particles.get(result).unwrap()
    );
}

fn run2(input_file: &str) {
    // Preamble
    let mut accelation_sums: Vec<i64> = Vec::new();
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut particles: Vec<Particles> = reader.lines().map(|i| i.unwrap().into()).collect();

    // dbg!(particles);
    for particle in particles.iter() {
        let accelation_sum =
            manhatten_distance_3d_from_zero(particle.a_x, particle.a_y, particle.a_z);
        accelation_sums.push(accelation_sum)
    }

    // Solve
    for _ in 0..40 {
        let mut hit_set: HashSet<(i64, i64, i64)> = HashSet::with_capacity(particles.len());
        let mut delete_set: HashSet<(i64, i64, i64)> = HashSet::new();
        for particel in particles.iter_mut() {
            particel.tick();
            if !hit_set.insert(particel.get_position()) {
                delete_set.insert(particel.get_position());
            }
        }

        for i in (0..particles.len()).rev() {
            if delete_set.contains(&particles[i].get_position()) {
                particles.remove(i);
            }
        }
    }

    // Result
    println!("Result of part 2 is {}", particles.len());
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
