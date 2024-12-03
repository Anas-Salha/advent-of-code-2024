use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing,
    Invalid,
}

// Define direction based on first two indices
fn get_direction(data: &Vec<u32>) -> Direction {
    match data[0] as i32 - data[1] as i32 {
        diff if diff > 0 => Direction::Decreasing,
        diff if diff < 0 => Direction::Increasing,
        _ => Direction::Invalid,
    }
}

fn is_gradual(data: &Vec<u32>) -> Option<usize> {
    data.windows(2).position(|w| match w[0].abs_diff(w[1]) {
        1..=3 => false,
        _ => true,
    })
}

fn is_uni_directional(data: &Vec<u32>, direction: Direction) -> Option<usize> {
    match direction {
        Direction::Decreasing => data.windows(2).position(|w| w[0] < w[1]),
        Direction::Increasing => data.windows(2).position(|w| w[0] > w[1]),
        Direction::Invalid => return Some(0),
    }
}

fn is_safe(data: Vec<u32>) -> bool {
    let direction = get_direction(&data);
    let result = is_uni_directional(&data, direction).or_else(|| is_gradual(&data));

    match result {
        None => true,
        Some(_) => false,
    }
}

fn main() {
    let filename = "data/input";
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    let safe_lines = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(' ')
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|data| is_safe(data.clone()))
        .count();

    dbg!(&safe_lines);
}
