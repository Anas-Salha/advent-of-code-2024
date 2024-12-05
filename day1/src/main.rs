use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = "data/input";
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) = reader
        .lines()
        .map_while(Result::ok)
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
        .map(|(index, value)| value.abs_diff(right_list[index]))
        .sum::<u32>();

    let similarity_score: u32 = left_list
        .iter()
        .map(|number| {
            let count = right_list.iter().filter(|x| *x == number).count() as u32;
            count * number
        })
        .sum::<u32>();

    println!("{total_distance}");
    println!("{similarity_score}");
}
