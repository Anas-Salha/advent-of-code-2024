use regex::Regex;
use std::fs;
fn main() {
    let path = "data/input";
    let data = fs::read_to_string(path).unwrap();
    let data = data.replace('\n', " ");

    let operations = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result: u32 = operations
        .captures_iter(&data)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum();

    dbg!(&result);
}
