use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = "data/main.input";
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let parts: Vec<&str> = line.split("   ").collect();
            let left: u32 = parts[0].parse().unwrap();
            let right: u32 = parts[1].parse().unwrap();
            (left, right)
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    let left_list = left_list;
    let right_list = right_list;

    let total_distance: u32 = left_list
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let distance = value.abs_diff(right_list[index]);
            distance
        })
        .sum::<u32>();

    println!("{total_distance}");
}
