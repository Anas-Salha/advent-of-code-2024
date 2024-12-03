use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

fn is_gradual(data: &Vec<u32>) -> bool {
    data.windows(2).all(|w| match w[0].abs_diff(w[1]) {
        1..=3 => true,
        _ => false,
    })
}

fn is_uni_directional(data: &Vec<u32>, direction: Direction) -> bool {
    match direction {
        Direction::Decreasing => data.windows(2).all(|w| w[0] > w[1]),
        Direction::Increasing => data.windows(2).all(|w| w[0] < w[1]),
    }
}

fn is_safe(data: &Vec<u32>) -> bool {
    (is_uni_directional(&data, Direction::Increasing)
        || is_uni_directional(&data, Direction::Decreasing))
        && is_gradual(&data)
}

fn is_safe_with_dampener(data: Vec<u32>) -> bool {
    if is_safe(&data) {
        return true;
    }

    (0..data.len()).any(|index| {
        let mut tmp_data = data.clone();
        tmp_data.remove(index);
        is_safe(&tmp_data)
    })
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
        .filter(|data| is_safe_with_dampener(data.clone()))
        .count();

    dbg!(&safe_lines);
}
